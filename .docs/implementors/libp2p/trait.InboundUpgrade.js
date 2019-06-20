(function() {var implementors = {};
implementors["libp2p"] = [{text:"impl&lt;C, F, O&gt; <a class=\"trait\" href=\"libp2p/trait.InboundUpgrade.html\" title=\"trait libp2p::InboundUpgrade\">InboundUpgrade</a>&lt;C&gt; for <a class=\"struct\" href=\"libp2p/simple/struct.SimpleProtocol.html\" title=\"struct libp2p::simple::SimpleProtocol\">SimpleProtocol</a>&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"struct\" href=\"libp2p/core/upgrade/struct.Negotiated.html\" title=\"struct libp2p::core::upgrade::Negotiated\">Negotiated</a>&lt;C&gt;) -&gt; O,<br>&nbsp;&nbsp;&nbsp;&nbsp;O: <a class=\"trait\" href=\"futures/future/trait.IntoFuture.html\" title=\"trait futures::future::IntoFuture\">IntoFuture</a>&lt;Error = <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">IoError</a>&gt;,&nbsp;</span>",synthetic:false,types:["libp2p::simple::SimpleProtocol"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
