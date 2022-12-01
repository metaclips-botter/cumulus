// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

// From construct_runtime macro
#![allow(clippy::from_over_into)]

use crate::{calc_relayers_rewards, Config};

use bp_messages::{
	source_chain::{LaneMessageVerifier, MessageDeliveryAndDispatchPayment, TargetHeaderChain},
	target_chain::{
		DispatchMessage, DispatchMessageData, MessageDispatch, ProvedLaneMessages, ProvedMessages,
		SourceHeaderChain,
	},
	DeliveredMessages, InboundLaneData, LaneId, Message, MessageKey, MessageNonce, MessagePayload,
	OutboundLaneData, UnrewardedRelayer,
};
use bp_runtime::{messages::MessageDispatchResult, Size};
use codec::{Decode, Encode};
use frame_support::{
	parameter_types,
	weights::{RuntimeDbWeight, Weight},
};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	testing::Header as SubstrateHeader,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};
use std::{
	collections::{BTreeMap, VecDeque},
	ops::RangeInclusive,
};

pub type AccountId = u64;
pub type Balance = u64;
#[derive(Decode, Encode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct TestPayload {
	/// Field that may be used to identify messages.
	pub id: u64,
	/// Reject this message by lane verifier?
	pub reject_by_lane_verifier: bool,
	/// Dispatch weight that is declared by the message sender.
	pub declared_weight: Weight,
	/// Message dispatch result.
	///
	/// Note: in correct code `dispatch_result.unspent_weight` will always be <= `declared_weight`,
	/// but for test purposes we'll be making it larger than `declared_weight` sometimes.
	pub dispatch_result: MessageDispatchResult<TestDispatchLevelResult>,
	/// Extra bytes that affect payload size.
	pub extra: Vec<u8>,
}
pub type TestMessageFee = u64;
pub type TestRelayer = u64;
pub type TestDispatchLevelResult = ();

type Block = frame_system::mocking::MockBlock<TestRuntime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;

use crate as pallet_bridge_messages;

frame_support::construct_runtime! {
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Event<T>},
		Messages: pallet_bridge_messages::{Pallet, Call, Event<T>},
	}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = Weight::from_ref_time(1024);
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
	pub const DbWeight: RuntimeDbWeight = RuntimeDbWeight { read: 1, write: 2 };
}

impl frame_system::Config for TestRuntime {
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type RuntimeCall = RuntimeCall;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = SubstrateHeader;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = DbWeight;
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for TestRuntime {
	type MaxLocks = ();
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<TestRuntime>;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

parameter_types! {
	pub const MaxMessagesToPruneAtOnce: u64 = 10;
	pub const MaxUnrewardedRelayerEntriesAtInboundLane: u64 = 16;
	pub const MaxUnconfirmedMessagesAtInboundLane: u64 = 32;
	pub const TestBridgedChainId: bp_runtime::ChainId = *b"test";
	pub const ActiveOutboundLanes: &'static [LaneId] = &[TEST_LANE_ID, TEST_LANE_ID_2];
}

impl Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type ActiveOutboundLanes = ActiveOutboundLanes;
	type MaxUnrewardedRelayerEntriesAtInboundLane = MaxUnrewardedRelayerEntriesAtInboundLane;
	type MaxUnconfirmedMessagesAtInboundLane = MaxUnconfirmedMessagesAtInboundLane;

	type MaximalOutboundPayloadSize = frame_support::traits::ConstU32<MAX_OUTBOUND_PAYLOAD_SIZE>;
	type OutboundPayload = TestPayload;

	type InboundPayload = TestPayload;
	type InboundRelayer = TestRelayer;

	type TargetHeaderChain = TestTargetHeaderChain;
	type LaneMessageVerifier = TestLaneMessageVerifier;
	type MessageDeliveryAndDispatchPayment = TestMessageDeliveryAndDispatchPayment;

	type SourceHeaderChain = TestSourceHeaderChain;
	type MessageDispatch = TestMessageDispatch;
	type BridgedChainId = TestBridgedChainId;
}

impl Size for TestPayload {
	fn size(&self) -> u32 {
		16 + self.extra.len() as u32
	}
}

/// Maximal outbound payload size.
pub const MAX_OUTBOUND_PAYLOAD_SIZE: u32 = 4096;

/// Account that has balance to use in tests.
pub const ENDOWED_ACCOUNT: AccountId = 0xDEAD;

/// Account id of test relayer.
pub const TEST_RELAYER_A: AccountId = 100;

/// Account id of additional test relayer - B.
pub const TEST_RELAYER_B: AccountId = 101;

/// Account id of additional test relayer - C.
pub const TEST_RELAYER_C: AccountId = 102;

/// Error that is returned by all test implementations.
pub const TEST_ERROR: &str = "Test error";

/// Lane that we're using in tests.
pub const TEST_LANE_ID: LaneId = [0, 0, 0, 1];

/// Secondary lane that we're using in tests.
pub const TEST_LANE_ID_2: LaneId = [0, 0, 0, 2];

/// Inactive outbound lane.
pub const TEST_LANE_ID_3: LaneId = [0, 0, 0, 3];

/// Regular message payload.
pub const REGULAR_PAYLOAD: TestPayload = message_payload(0, 50);

/// Payload that is rejected by `TestTargetHeaderChain`.
pub const PAYLOAD_REJECTED_BY_TARGET_CHAIN: TestPayload = message_payload(1, 50);

/// Vec of proved messages, grouped by lane.
pub type MessagesByLaneVec = Vec<(LaneId, ProvedLaneMessages<Message>)>;

/// Test messages proof.
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct TestMessagesProof {
	pub result: Result<MessagesByLaneVec, ()>,
}

impl Size for TestMessagesProof {
	fn size(&self) -> u32 {
		0
	}
}

impl From<Result<Vec<Message>, ()>> for TestMessagesProof {
	fn from(result: Result<Vec<Message>, ()>) -> Self {
		Self {
			result: result.map(|messages| {
				let mut messages_by_lane: BTreeMap<LaneId, ProvedLaneMessages<Message>> =
					BTreeMap::new();
				for message in messages {
					messages_by_lane.entry(message.key.lane_id).or_default().messages.push(message);
				}
				messages_by_lane.into_iter().collect()
			}),
		}
	}
}

/// Messages delivery proof used in tests.
#[derive(Debug, Encode, Decode, Eq, Clone, PartialEq, TypeInfo)]
pub struct TestMessagesDeliveryProof(pub Result<(LaneId, InboundLaneData<TestRelayer>), ()>);

impl Size for TestMessagesDeliveryProof {
	fn size(&self) -> u32 {
		0
	}
}

/// Target header chain that is used in tests.
#[derive(Debug, Default)]
pub struct TestTargetHeaderChain;

impl TargetHeaderChain<TestPayload, TestRelayer> for TestTargetHeaderChain {
	type Error = &'static str;

	type MessagesDeliveryProof = TestMessagesDeliveryProof;

	fn verify_message(payload: &TestPayload) -> Result<(), Self::Error> {
		if *payload == PAYLOAD_REJECTED_BY_TARGET_CHAIN {
			Err(TEST_ERROR)
		} else {
			Ok(())
		}
	}

	fn verify_messages_delivery_proof(
		proof: Self::MessagesDeliveryProof,
	) -> Result<(LaneId, InboundLaneData<TestRelayer>), Self::Error> {
		proof.0.map_err(|_| TEST_ERROR)
	}
}

/// Lane message verifier that is used in tests.
#[derive(Debug, Default)]
pub struct TestLaneMessageVerifier;

impl LaneMessageVerifier<RuntimeOrigin, TestPayload> for TestLaneMessageVerifier {
	type Error = &'static str;

	fn verify_message(
		_submitter: &RuntimeOrigin,
		_lane: &LaneId,
		_lane_outbound_data: &OutboundLaneData,
		payload: &TestPayload,
	) -> Result<(), Self::Error> {
		if !payload.reject_by_lane_verifier {
			Ok(())
		} else {
			Err(TEST_ERROR)
		}
	}
}

/// Message fee payment system that is used in tests.
#[derive(Debug, Default)]
pub struct TestMessageDeliveryAndDispatchPayment;

impl TestMessageDeliveryAndDispatchPayment {
	/// Returns true if given relayer has been rewarded with given balance. The reward-paid flag is
	/// cleared after the call.
	pub fn is_reward_paid(relayer: AccountId, fee: TestMessageFee) -> bool {
		let key = (b":relayer-reward:", relayer, fee).encode();
		frame_support::storage::unhashed::take::<bool>(&key).is_some()
	}
}

impl MessageDeliveryAndDispatchPayment<RuntimeOrigin, AccountId>
	for TestMessageDeliveryAndDispatchPayment
{
	type Error = &'static str;

	fn pay_relayers_rewards(
		_lane_id: LaneId,
		message_relayers: VecDeque<UnrewardedRelayer<AccountId>>,
		_confirmation_relayer: &AccountId,
		received_range: &RangeInclusive<MessageNonce>,
	) {
		let relayers_rewards =
			calc_relayers_rewards::<TestRuntime, ()>(message_relayers, received_range);
		for (relayer, reward) in &relayers_rewards {
			let key = (b":relayer-reward:", relayer, reward).encode();
			frame_support::storage::unhashed::put(&key, &true);
		}
	}
}

/// Source header chain that is used in tests.
#[derive(Debug)]
pub struct TestSourceHeaderChain;

impl SourceHeaderChain for TestSourceHeaderChain {
	type Error = &'static str;

	type MessagesProof = TestMessagesProof;

	fn verify_messages_proof(
		proof: Self::MessagesProof,
		_messages_count: u32,
	) -> Result<ProvedMessages<Message>, Self::Error> {
		proof.result.map(|proof| proof.into_iter().collect()).map_err(|_| TEST_ERROR)
	}
}

/// Source header chain that is used in tests.
#[derive(Debug)]
pub struct TestMessageDispatch;

impl MessageDispatch<AccountId> for TestMessageDispatch {
	type DispatchPayload = TestPayload;
	type DispatchLevelResult = TestDispatchLevelResult;

	fn dispatch_weight(message: &mut DispatchMessage<TestPayload>) -> Weight {
		match message.data.payload.as_ref() {
			Ok(payload) => payload.declared_weight,
			Err(_) => Weight::from_ref_time(0),
		}
	}

	fn dispatch(
		_relayer_account: &AccountId,
		message: DispatchMessage<TestPayload>,
	) -> MessageDispatchResult<TestDispatchLevelResult> {
		match message.data.payload.as_ref() {
			Ok(payload) => payload.dispatch_result.clone(),
			Err(_) => dispatch_result(0),
		}
	}
}

/// Return test lane message with given nonce and payload.
pub fn message(nonce: MessageNonce, payload: TestPayload) -> Message {
	Message { key: MessageKey { lane_id: TEST_LANE_ID, nonce }, payload: payload.encode() }
}

/// Return valid outbound message data, constructed from given payload.
pub fn outbound_message_data(payload: TestPayload) -> MessagePayload {
	payload.encode()
}

/// Return valid inbound (dispatch) message data, constructed from given payload.
pub fn inbound_message_data(payload: TestPayload) -> DispatchMessageData<TestPayload> {
	DispatchMessageData { payload: Ok(payload) }
}

/// Constructs message payload using given arguments and zero unspent weight.
pub const fn message_payload(id: u64, declared_weight: u64) -> TestPayload {
	TestPayload {
		id,
		reject_by_lane_verifier: false,
		declared_weight: Weight::from_ref_time(declared_weight),
		dispatch_result: dispatch_result(0),
		extra: Vec::new(),
	}
}

/// Returns message dispatch result with given unspent weight.
pub const fn dispatch_result(
	unspent_weight: u64,
) -> MessageDispatchResult<TestDispatchLevelResult> {
	MessageDispatchResult {
		unspent_weight: Weight::from_ref_time(unspent_weight),
		dispatch_fee_paid_during_dispatch: true,
		dispatch_level_result: (),
	}
}

/// Constructs unrewarded relayer entry from nonces range and relayer id.
pub fn unrewarded_relayer(
	begin: MessageNonce,
	end: MessageNonce,
	relayer: TestRelayer,
) -> UnrewardedRelayer<TestRelayer> {
	UnrewardedRelayer { relayer, messages: DeliveredMessages { begin, end } }
}

/// Run pallet test.
pub fn run_test<T>(test: impl FnOnce() -> T) -> T {
	let mut t = frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
	pallet_balances::GenesisConfig::<TestRuntime> { balances: vec![(ENDOWED_ACCOUNT, 1_000_000)] }
		.assimilate_storage(&mut t)
		.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(test)
}
