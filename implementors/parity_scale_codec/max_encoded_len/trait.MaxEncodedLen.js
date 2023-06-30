(function() {var implementors = {
"asset_hub_kusama_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_kusama_runtime/enum.RuntimeHoldReason.html\" title=\"enum asset_hub_kusama_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_kusama_runtime/enum.RuntimeFreezeReason.html\" title=\"enum asset_hub_kusama_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_kusama_runtime/enum.OriginCaller.html\" title=\"enum asset_hub_kusama_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_kusama_runtime/enum.RuntimeLockId.html\" title=\"enum asset_hub_kusama_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_kusama_runtime/enum.ProxyType.html\" title=\"enum asset_hub_kusama_runtime::ProxyType\">ProxyType</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_kusama_runtime/enum.RuntimeSlashReason.html\" title=\"enum asset_hub_kusama_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"]],
"asset_hub_polkadot_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_polkadot_runtime/enum.RuntimeSlashReason.html\" title=\"enum asset_hub_polkadot_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_polkadot_runtime/enum.RuntimeFreezeReason.html\" title=\"enum asset_hub_polkadot_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_polkadot_runtime/enum.OriginCaller.html\" title=\"enum asset_hub_polkadot_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_polkadot_runtime/enum.ProxyType.html\" title=\"enum asset_hub_polkadot_runtime::ProxyType\">ProxyType</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_polkadot_runtime/enum.RuntimeHoldReason.html\" title=\"enum asset_hub_polkadot_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_polkadot_runtime/enum.RuntimeLockId.html\" title=\"enum asset_hub_polkadot_runtime::RuntimeLockId\">RuntimeLockId</a>"]],
"asset_hub_westend_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_westend_runtime/enum.ProxyType.html\" title=\"enum asset_hub_westend_runtime::ProxyType\">ProxyType</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_westend_runtime/enum.OriginCaller.html\" title=\"enum asset_hub_westend_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_westend_runtime/enum.RuntimeLockId.html\" title=\"enum asset_hub_westend_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_westend_runtime/enum.RuntimeHoldReason.html\" title=\"enum asset_hub_westend_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_westend_runtime/enum.RuntimeSlashReason.html\" title=\"enum asset_hub_westend_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"asset_hub_westend_runtime/enum.RuntimeFreezeReason.html\" title=\"enum asset_hub_westend_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"]],
"bp_header_chain":[["impl&lt;Number, Hash&gt; MaxEncodedLen for <a class=\"struct\" href=\"bp_header_chain/struct.StoredHeaderData.html\" title=\"struct bp_header_chain::StoredHeaderData\">StoredHeaderData</a>&lt;Number, Hash&gt;<span class=\"where fmt-newline\">where\n    Number: MaxEncodedLen,\n    Hash: MaxEncodedLen,</span>"]],
"bp_messages":[["impl&lt;RelayerId&gt; MaxEncodedLen for <a class=\"struct\" href=\"bp_messages/struct.UnrewardedRelayer.html\" title=\"struct bp_messages::UnrewardedRelayer\">UnrewardedRelayer</a>&lt;RelayerId&gt;<span class=\"where fmt-newline\">where\n    RelayerId: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_messages/struct.DeliveredMessages.html\" title=\"struct bp_messages::DeliveredMessages\">DeliveredMessages</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_messages/struct.OutboundLaneData.html\" title=\"struct bp_messages::OutboundLaneData\">OutboundLaneData</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_messages/struct.MessageKey.html\" title=\"struct bp_messages::MessageKey\">MessageKey</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_messages/struct.LaneId.html\" title=\"struct bp_messages::LaneId\">LaneId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bp_messages/enum.MessagesOperatingMode.html\" title=\"enum bp_messages::MessagesOperatingMode\">MessagesOperatingMode</a>"]],
"bp_parachains":[["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_parachains/struct.ParaInfo.html\" title=\"struct bp_parachains::ParaInfo\">ParaInfo</a>"],["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_parachains/struct.BestParaHeadHash.html\" title=\"struct bp_parachains::BestParaHeadHash\">BestParaHeadHash</a>"]],
"bp_polkadot_core":[["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_polkadot_core/parachains/struct.ParaId.html\" title=\"struct bp_polkadot_core::parachains::ParaId\">ParaId</a>"]],
"bp_relayers":[["impl MaxEncodedLen for <a class=\"struct\" href=\"bp_relayers/struct.RewardsAccountParams.html\" title=\"struct bp_relayers::RewardsAccountParams\">RewardsAccountParams</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bp_relayers/enum.RewardsAccountOwner.html\" title=\"enum bp_relayers::RewardsAccountOwner\">RewardsAccountOwner</a>"],["impl&lt;BlockNumber, Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"bp_relayers/struct.Registration.html\" title=\"struct bp_relayers::Registration\">Registration</a>&lt;BlockNumber, Balance&gt;<span class=\"where fmt-newline\">where\n    BlockNumber: MaxEncodedLen,\n    Balance: MaxEncodedLen,</span>"]],
"bp_runtime":[["impl&lt;Hash, Number&gt; MaxEncodedLen for <a class=\"struct\" href=\"bp_runtime/struct.HeaderId.html\" title=\"struct bp_runtime::HeaderId\">HeaderId</a>&lt;Hash, Number&gt;<span class=\"where fmt-newline\">where\n    Number: MaxEncodedLen,\n    Hash: MaxEncodedLen,</span>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bp_runtime/enum.BasicOperatingMode.html\" title=\"enum bp_runtime::BasicOperatingMode\">BasicOperatingMode</a>"],["impl&lt;B: Get&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.u32.html\">u32</a>&gt;, V: Encode&gt; MaxEncodedLen for <a class=\"struct\" href=\"bp_runtime/struct.BoundedStorageValue.html\" title=\"struct bp_runtime::BoundedStorageValue\">BoundedStorageValue</a>&lt;B, V&gt;"]],
"bridge_hub_kusama_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_kusama_runtime/enum.RuntimeLockId.html\" title=\"enum bridge_hub_kusama_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_kusama_runtime/enum.RuntimeHoldReason.html\" title=\"enum bridge_hub_kusama_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_kusama_runtime/enum.RuntimeFreezeReason.html\" title=\"enum bridge_hub_kusama_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_kusama_runtime/enum.OriginCaller.html\" title=\"enum bridge_hub_kusama_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_kusama_runtime/enum.RuntimeSlashReason.html\" title=\"enum bridge_hub_kusama_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"]],
"bridge_hub_polkadot_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_polkadot_runtime/enum.RuntimeHoldReason.html\" title=\"enum bridge_hub_polkadot_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_polkadot_runtime/enum.RuntimeFreezeReason.html\" title=\"enum bridge_hub_polkadot_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_polkadot_runtime/enum.RuntimeLockId.html\" title=\"enum bridge_hub_polkadot_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_polkadot_runtime/enum.OriginCaller.html\" title=\"enum bridge_hub_polkadot_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_polkadot_runtime/enum.RuntimeSlashReason.html\" title=\"enum bridge_hub_polkadot_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"]],
"bridge_hub_rococo_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_rococo_runtime/enum.RuntimeSlashReason.html\" title=\"enum bridge_hub_rococo_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_rococo_runtime/enum.RuntimeHoldReason.html\" title=\"enum bridge_hub_rococo_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_rococo_runtime/enum.OriginCaller.html\" title=\"enum bridge_hub_rococo_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_rococo_runtime/enum.RuntimeLockId.html\" title=\"enum bridge_hub_rococo_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"bridge_hub_rococo_runtime/enum.RuntimeFreezeReason.html\" title=\"enum bridge_hub_rococo_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"]],
"collectives_polkadot_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"collectives_polkadot_runtime/enum.RuntimeFreezeReason.html\" title=\"enum collectives_polkadot_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"collectives_polkadot_runtime/enum.RuntimeLockId.html\" title=\"enum collectives_polkadot_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"collectives_polkadot_runtime/enum.ProxyType.html\" title=\"enum collectives_polkadot_runtime::ProxyType\">ProxyType</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"collectives_polkadot_runtime/fellowship/pallet_fellowship_origins/enum.Origin.html\" title=\"enum collectives_polkadot_runtime::fellowship::pallet_fellowship_origins::Origin\">Origin</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"collectives_polkadot_runtime/enum.RuntimeHoldReason.html\" title=\"enum collectives_polkadot_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"collectives_polkadot_runtime/enum.RuntimeSlashReason.html\" title=\"enum collectives_polkadot_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"collectives_polkadot_runtime/enum.OriginCaller.html\" title=\"enum collectives_polkadot_runtime::OriginCaller\">OriginCaller</a>"]],
"contracts_rococo_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"contracts_rococo_runtime/enum.OriginCaller.html\" title=\"enum contracts_rococo_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"contracts_rococo_runtime/enum.RuntimeLockId.html\" title=\"enum contracts_rococo_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"contracts_rococo_runtime/enum.RuntimeHoldReason.html\" title=\"enum contracts_rococo_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"contracts_rococo_runtime/enum.RuntimeFreezeReason.html\" title=\"enum contracts_rococo_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"contracts_rococo_runtime/enum.RuntimeSlashReason.html\" title=\"enum contracts_rococo_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"]],
"cumulus_pallet_xcm":[["impl MaxEncodedLen for <a class=\"enum\" href=\"cumulus_pallet_xcm/pallet/enum.Origin.html\" title=\"enum cumulus_pallet_xcm::pallet::Origin\">Origin</a>"]],
"cumulus_test_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"cumulus_test_runtime/enum.RuntimeSlashReason.html\" title=\"enum cumulus_test_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"cumulus_test_runtime/enum.OriginCaller.html\" title=\"enum cumulus_test_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"cumulus_test_runtime/enum.RuntimeLockId.html\" title=\"enum cumulus_test_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"cumulus_test_runtime/enum.RuntimeHoldReason.html\" title=\"enum cumulus_test_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"cumulus_test_runtime/enum.RuntimeFreezeReason.html\" title=\"enum cumulus_test_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"]],
"glutton_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"glutton_runtime/enum.RuntimeHoldReason.html\" title=\"enum glutton_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"glutton_runtime/enum.RuntimeLockId.html\" title=\"enum glutton_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"glutton_runtime/enum.RuntimeFreezeReason.html\" title=\"enum glutton_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"glutton_runtime/enum.OriginCaller.html\" title=\"enum glutton_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"glutton_runtime/enum.RuntimeSlashReason.html\" title=\"enum glutton_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"]],
"pallet_bridge_grandpa":[["impl&lt;T: <a class=\"trait\" href=\"pallet_bridge_grandpa/pallet/trait.Config.html\" title=\"trait pallet_bridge_grandpa::pallet::Config\">Config</a>&lt;I&gt;, I: 'static&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_bridge_grandpa/struct.StoredAuthoritySet.html\" title=\"struct pallet_bridge_grandpa::StoredAuthoritySet\">StoredAuthoritySet</a>&lt;T, I&gt;<span class=\"where fmt-newline\">where\n    BoundedVec&lt;(AuthorityId, AuthorityWeight), StoredAuthorityListLimit&lt;T, I&gt;&gt;: MaxEncodedLen,</span>"]],
"pallet_bridge_messages":[["impl&lt;T: <a class=\"trait\" href=\"pallet_bridge_messages/pallet/trait.Config.html\" title=\"trait pallet_bridge_messages::pallet::Config\">Config</a>&lt;I&gt;, I: 'static&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_bridge_messages/struct.StoredInboundLaneData.html\" title=\"struct pallet_bridge_messages::StoredInboundLaneData\">StoredInboundLaneData</a>&lt;T, I&gt;"]],
"pallet_collator_selection":[["impl&lt;AccountId, Balance&gt; MaxEncodedLen for <a class=\"struct\" href=\"pallet_collator_selection/pallet/struct.CandidateInfo.html\" title=\"struct pallet_collator_selection::pallet::CandidateInfo\">CandidateInfo</a>&lt;AccountId, Balance&gt;<span class=\"where fmt-newline\">where\n    AccountId: MaxEncodedLen,\n    Balance: MaxEncodedLen,</span>"]],
"parachain_template_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"parachain_template_runtime/enum.RuntimeLockId.html\" title=\"enum parachain_template_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"parachain_template_runtime/enum.RuntimeFreezeReason.html\" title=\"enum parachain_template_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"parachain_template_runtime/enum.OriginCaller.html\" title=\"enum parachain_template_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"parachain_template_runtime/enum.RuntimeSlashReason.html\" title=\"enum parachain_template_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"parachain_template_runtime/enum.RuntimeHoldReason.html\" title=\"enum parachain_template_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"]],
"penpal_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"penpal_runtime/enum.RuntimeHoldReason.html\" title=\"enum penpal_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"penpal_runtime/enum.RuntimeLockId.html\" title=\"enum penpal_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"penpal_runtime/enum.RuntimeSlashReason.html\" title=\"enum penpal_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"penpal_runtime/enum.OriginCaller.html\" title=\"enum penpal_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"penpal_runtime/enum.RuntimeFreezeReason.html\" title=\"enum penpal_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"]],
"rococo_parachain_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"rococo_parachain_runtime/enum.RuntimeLockId.html\" title=\"enum rococo_parachain_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"rococo_parachain_runtime/enum.RuntimeHoldReason.html\" title=\"enum rococo_parachain_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"rococo_parachain_runtime/enum.RuntimeSlashReason.html\" title=\"enum rococo_parachain_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"rococo_parachain_runtime/enum.OriginCaller.html\" title=\"enum rococo_parachain_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"rococo_parachain_runtime/enum.RuntimeFreezeReason.html\" title=\"enum rococo_parachain_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"]],
"seedling_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"seedling_runtime/enum.RuntimeLockId.html\" title=\"enum seedling_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"seedling_runtime/enum.OriginCaller.html\" title=\"enum seedling_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"seedling_runtime/enum.RuntimeHoldReason.html\" title=\"enum seedling_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"seedling_runtime/enum.RuntimeSlashReason.html\" title=\"enum seedling_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"seedling_runtime/enum.RuntimeFreezeReason.html\" title=\"enum seedling_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"]],
"shell_runtime":[["impl MaxEncodedLen for <a class=\"enum\" href=\"shell_runtime/enum.RuntimeHoldReason.html\" title=\"enum shell_runtime::RuntimeHoldReason\">RuntimeHoldReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"shell_runtime/enum.RuntimeSlashReason.html\" title=\"enum shell_runtime::RuntimeSlashReason\">RuntimeSlashReason</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"shell_runtime/enum.OriginCaller.html\" title=\"enum shell_runtime::OriginCaller\">OriginCaller</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"shell_runtime/enum.RuntimeLockId.html\" title=\"enum shell_runtime::RuntimeLockId\">RuntimeLockId</a>"],["impl MaxEncodedLen for <a class=\"enum\" href=\"shell_runtime/enum.RuntimeFreezeReason.html\" title=\"enum shell_runtime::RuntimeFreezeReason\">RuntimeFreezeReason</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()