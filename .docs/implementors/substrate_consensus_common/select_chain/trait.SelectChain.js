(function() {var implementors = {};
implementors["substrate_client"] = [{text:"impl&lt;B, Block&gt; <a class=\"trait\" href=\"substrate_consensus_common/select_chain/trait.SelectChain.html\" title=\"trait substrate_consensus_common::select_chain::SelectChain\">SelectChain</a>&lt;Block&gt; for <a class=\"struct\" href=\"substrate_client/struct.LongestChain.html\" title=\"struct substrate_client::LongestChain\">LongestChain</a>&lt;B, Block&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"substrate_client/backend/trait.Backend.html\" title=\"trait substrate_client::backend::Backend\">Backend</a>&lt;Block, <a class=\"struct\" href=\"substrate_primitives/hasher/blake2/struct.Blake2Hasher.html\" title=\"struct substrate_primitives::hasher::blake2::Blake2Hasher\">Blake2Hasher</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Block: <a class=\"trait\" href=\"sr_primitives/traits/trait.Block.html\" title=\"trait sr_primitives::traits::Block\">BlockT</a>&lt;Hash = <a class=\"struct\" href=\"primitive_types/struct.H256.html\" title=\"struct primitive_types::H256\">H256</a>&gt;,&nbsp;</span>",synthetic:false,types:["substrate_client::client::LongestChain"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
