(function() {var implementors = {
"heapless":[["impl&lt;T, const N: usize&gt; Extend&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;"],["impl&lt;T, S, const N: usize&gt; Extend&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, S, N&gt;<span class=\"where fmt-newline\">where\n    T: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;T, const N: usize&gt; Extend&lt;T&gt; for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt;"],["impl&lt;'a, K, V, S, const N: usize&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, S, N&gt;<span class=\"where fmt-newline\">where\n    K: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + Copy,\n    V: Copy,\n    S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;'a, T, S, const N: usize&gt; Extend&lt;&amp;'a T&gt; for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, S, N&gt;<span class=\"where fmt-newline\">where\n    T: 'a + Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + Copy,\n    S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;'a, T, const N: usize&gt; Extend&lt;&amp;'a T&gt; for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where\n    T: 'a + Copy,</span>"],["impl&lt;'a, T, const N: usize&gt; Extend&lt;&amp;'a T&gt; for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where\n    T: 'a + Clone,</span>"],["impl&lt;K, V, S, const N: usize&gt; Extend&lt;(K, V)&gt; for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, S, N&gt;<span class=\"where fmt-newline\">where\n    K: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a>,\n    S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()