(function() {var implementors = {
"arduboy_rust":[["impl Debug for <a class=\"struct\" href=\"arduboy_rust/prelude/struct.ButtonSet.html\" title=\"struct arduboy_rust::prelude::ButtonSet\">ButtonSet</a>"],["impl Debug for <a class=\"enum\" href=\"arduboy_rust/prelude/enum.Base.html\" title=\"enum arduboy_rust::prelude::Base\">Base</a>"],["impl Debug for <a class=\"struct\" href=\"arduboy_rust/prelude/arduboy2/struct.Point.html\" title=\"struct arduboy_rust::prelude::arduboy2::Point\">Point</a>"],["impl Debug for <a class=\"struct\" href=\"arduboy_rust/prelude/arduboy2/struct.Rect.html\" title=\"struct arduboy_rust::prelude::arduboy2::Rect\">Rect</a>"],["impl Debug for <a class=\"enum\" href=\"arduboy_rust/prelude/arduboy2/enum.Color.html\" title=\"enum arduboy_rust::prelude::arduboy2::Color\">Color</a>"]],
"byteorder":[["impl Debug for <a class=\"enum\" href=\"byteorder/enum.BigEndian.html\" title=\"enum byteorder::BigEndian\">BigEndian</a>"],["impl Debug for <a class=\"enum\" href=\"byteorder/enum.LittleEndian.html\" title=\"enum byteorder::LittleEndian\">LittleEndian</a>"]],
"critical_section":[["impl&lt;T: Debug&gt; Debug for <a class=\"struct\" href=\"critical_section/struct.Mutex.html\" title=\"struct critical_section::Mutex\">Mutex</a>&lt;T&gt;"],["impl&lt;'cs&gt; Debug for <a class=\"struct\" href=\"critical_section/struct.CriticalSection.html\" title=\"struct critical_section::CriticalSection\">CriticalSection</a>&lt;'cs&gt;"],["impl Debug for <a class=\"struct\" href=\"critical_section/struct.RestoreState.html\" title=\"struct critical_section::RestoreState\">RestoreState</a>"]],
"drboy":[["impl Debug for <a class=\"struct\" href=\"drboy/struct.Enemy.html\" title=\"struct drboy::Enemy\">Enemy</a>"],["impl Debug for <a class=\"struct\" href=\"drboy/struct.Player.html\" title=\"struct drboy::Player\">Player</a>"],["impl Debug for <a class=\"enum\" href=\"drboy/enum.GameMode.html\" title=\"enum drboy::GameMode\">GameMode</a>"]],
"hash32":[["impl&lt;H: Default + <a class=\"trait\" href=\"hash32/trait.Hasher.html\" title=\"trait hash32::Hasher\">Hasher</a>&gt; Debug for <a class=\"struct\" href=\"hash32/struct.BuildHasherDefault.html\" title=\"struct hash32::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;"]],
"heapless":[["impl Debug for <a class=\"struct\" href=\"heapless/sorted_linked_list/struct.LinkedIndexUsize.html\" title=\"struct heapless::sorted_linked_list::LinkedIndexUsize\">LinkedIndexUsize</a>"],["impl&lt;T, Idx, K, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/sorted_linked_list/struct.SortedLinkedList.html\" title=\"struct heapless::sorted_linked_list::SortedLinkedList\">SortedLinkedList</a>&lt;T, Idx, K, N&gt;<span class=\"where fmt-newline\">where\n    T: Ord + Debug,\n    Idx: <a class=\"trait\" href=\"heapless/sorted_linked_list/trait.SortedLinkedListIndex.html\" title=\"trait heapless::sorted_linked_list::SortedLinkedListIndex\">SortedLinkedListIndex</a>,\n    K: <a class=\"trait\" href=\"heapless/sorted_linked_list/trait.Kind.html\" title=\"trait heapless::sorted_linked_list::Kind\">Kind</a>,</span>"],["impl&lt;T, S, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/struct.IndexSet.html\" title=\"struct heapless::IndexSet\">IndexSet</a>&lt;T, S, N&gt;<span class=\"where fmt-newline\">where\n    T: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + Debug,\n    S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"],["impl&lt;K, V, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/struct.LinearMap.html\" title=\"struct heapless::LinearMap\">LinearMap</a>&lt;K, V, N&gt;<span class=\"where fmt-newline\">where\n    K: Eq + Debug,\n    V: Debug,</span>"],["impl&lt;const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/struct.String.html\" title=\"struct heapless::String\">String</a>&lt;N&gt;"],["impl Debug for <a class=\"struct\" href=\"heapless/sorted_linked_list/struct.LinkedIndexU16.html\" title=\"struct heapless::sorted_linked_list::LinkedIndexU16\">LinkedIndexU16</a>"],["impl Debug for <a class=\"struct\" href=\"heapless/sorted_linked_list/struct.LinkedIndexU8.html\" title=\"struct heapless::sorted_linked_list::LinkedIndexU8\">LinkedIndexU8</a>"],["impl&lt;T, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/struct.HistoryBuffer.html\" title=\"struct heapless::HistoryBuffer\">HistoryBuffer</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where\n    T: Debug,</span>"],["impl&lt;T, K, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/binary_heap/struct.BinaryHeap.html\" title=\"struct heapless::binary_heap::BinaryHeap\">BinaryHeap</a>&lt;T, K, N&gt;<span class=\"where fmt-newline\">where\n    K: <a class=\"trait\" href=\"heapless/binary_heap/trait.Kind.html\" title=\"trait heapless::binary_heap::Kind\">Kind</a>,\n    T: Ord + Debug,</span>"],["impl&lt;T, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/struct.Vec.html\" title=\"struct heapless::Vec\">Vec</a>&lt;T, N&gt;<span class=\"where fmt-newline\">where\n    T: Debug,</span>"],["impl&lt;T: Debug, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/struct.Deque.html\" title=\"struct heapless::Deque\">Deque</a>&lt;T, N&gt;"],["impl&lt;K, V, S, const N: usize&gt; Debug for <a class=\"struct\" href=\"heapless/struct.IndexMap.html\" title=\"struct heapless::IndexMap\">IndexMap</a>&lt;K, V, S, N&gt;<span class=\"where fmt-newline\">where\n    K: Eq + <a class=\"trait\" href=\"hash32/trait.Hash.html\" title=\"trait hash32::Hash\">Hash</a> + Debug,\n    V: Debug,\n    S: <a class=\"trait\" href=\"hash32/trait.BuildHasher.html\" title=\"trait hash32::BuildHasher\">BuildHasher</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()