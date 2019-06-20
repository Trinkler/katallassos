(function() {var implementors = {};
implementors["substrate_transaction_pool"] = [{text:"impl&lt;T, Block&gt; <a class=\"trait\" href=\"substrate_transaction_graph/pool/trait.ChainApi.html\" title=\"trait substrate_transaction_graph::pool::ChainApi\">ChainApi</a> for <a class=\"struct\" href=\"substrate_transaction_pool/struct.ChainApi.html\" title=\"struct substrate_transaction_pool::ChainApi\">ChainApi</a>&lt;T, Block&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sr_primitives/traits/trait.Block.html\" title=\"trait sr_primitives::traits::Block\">Block</a>&lt;Hash = <a class=\"struct\" href=\"primitive_types/struct.H256.html\" title=\"struct primitive_types::H256\">H256</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"sr_primitives/traits/trait.ProvideRuntimeApi.html\" title=\"trait sr_primitives::traits::ProvideRuntimeApi\">ProvideRuntimeApi</a> + <a class=\"trait\" href=\"substrate_client/blockchain/trait.HeaderBackend.html\" title=\"trait substrate_client::blockchain::HeaderBackend\">HeaderBackend</a>&lt;Block&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;T::<a class=\"type\" href=\"sr_primitives/traits/trait.ProvideRuntimeApi.html#associatedtype.Api\" title=\"type sr_primitives::traits::ProvideRuntimeApi::Api\">Api</a>: <a class=\"trait\" href=\"substrate_client/runtime_api/trait.TaggedTransactionQueue.html\" title=\"trait substrate_client::runtime_api::TaggedTransactionQueue\">TaggedTransactionQueue</a>&lt;Block&gt;,&nbsp;</span>",synthetic:false,types:["substrate_transaction_pool::api::ChainApi"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
