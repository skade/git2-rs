(function() {var implementors = {};
implementors['rustc-serialize'] = ["<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;T: <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a href='http://doc.rust-lang.org/nightly/core/primitive.char.html'>char</a>&gt;&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='enum' href='rustc-serialize/json/enum.JsonEvent.html' title='rustc-serialize::json::JsonEvent'>JsonEvent</a>&gt; for <a class='struct' href='rustc-serialize/json/struct.Parser.html' title='rustc-serialize::json::Parser'>Parser</a>&lt;T&gt;",];
implementors['git2'] = ["<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'repo&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.tuple.html'>(<a class='struct' href='git2/struct.Branch.html' title='git2::Branch'>Branch</a>&lt;'repo&gt;, <a class='enum' href='git2/enum.BranchType.html' title='git2::BranchType'>BranchType</a>)</a>&gt; for <a class='struct' href='git2/struct.Branches.html' title='git2::Branches'>Branches</a>&lt;'repo&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'repo, 'commit&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.Commit.html' title='git2::Commit'>Commit</a>&lt;'repo&gt;&gt; for <a class='struct' href='git2/struct.Parents.html' title='git2::Parents'>Parents</a>&lt;'commit, 'repo&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'cfg, 'b&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.ConfigEntry.html' title='git2::ConfigEntry'>ConfigEntry</a>&lt;'b&gt;&gt; for &amp;'b <a class='struct' href='git2/struct.ConfigEntries.html' title='git2::ConfigEntries'>ConfigEntries</a>&lt;'cfg&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'diff&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.DiffDelta.html' title='git2::DiffDelta'>DiffDelta</a>&lt;'diff&gt;&gt; for <a class='struct' href='git2/struct.Deltas.html' title='git2::Deltas'>Deltas</a>&lt;'diff&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'index&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.IndexEntry.html' title='git2::IndexEntry'>IndexEntry</a>&gt; for <a class='struct' href='git2/struct.IndexEntries.html' title='git2::IndexEntries'>IndexEntries</a>&lt;'index&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'repo&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.tuple.html'>(<a class='struct' href='git2/struct.Oid.html' title='git2::Oid'>Oid</a>, <a class='struct' href='git2/struct.Oid.html' title='git2::Oid'>Oid</a>)</a>&gt; for <a class='struct' href='git2/struct.Notes.html' title='git2::Notes'>Notes</a>&lt;'repo&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'list&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>&amp;'list [</a><a href='http://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a><a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>]</a>&gt; for <a class='struct' href='git2/struct.PathspecEntries.html' title='git2::PathspecEntries'>PathspecEntries</a>&lt;'list&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'list&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.DiffDelta.html' title='git2::DiffDelta'>DiffDelta</a>&lt;'list&gt;&gt; for <a class='struct' href='git2/struct.PathspecDiffEntries.html' title='git2::PathspecDiffEntries'>PathspecDiffEntries</a>&lt;'list&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'list&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>&amp;'list [</a><a href='http://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a><a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>]</a>&gt; for <a class='struct' href='git2/struct.PathspecFailedEntries.html' title='git2::PathspecFailedEntries'>PathspecFailedEntries</a>&lt;'list&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'repo&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.Reference.html' title='git2::Reference'>Reference</a>&lt;'repo&gt;&gt; for <a class='struct' href='git2/struct.References.html' title='git2::References'>References</a>&lt;'repo&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'repo&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;&amp;'repo <a href='http://doc.rust-lang.org/nightly/std/primitive.str.html'>str</a>&gt; for <a class='struct' href='git2/struct.ReferenceNames.html' title='git2::ReferenceNames'>ReferenceNames</a>&lt;'repo&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'a, 'b&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.Refspec.html' title='git2::Refspec'>Refspec</a>&lt;'a&gt;&gt; for <a class='struct' href='git2/struct.Refspecs.html' title='git2::Refspecs'>Refspecs</a>&lt;'a, 'b&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'repo&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.Oid.html' title='git2::Oid'>Oid</a>&gt; for <a class='struct' href='git2/struct.Revwalk.html' title='git2::Revwalk'>Revwalk</a>&lt;'repo&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'a&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='struct' href='git2/struct.StatusEntry.html' title='git2::StatusEntry'>StatusEntry</a>&lt;'a&gt;&gt; for <a class='struct' href='git2/struct.StatusIter.html' title='git2::StatusIter'>StatusIter</a>&lt;'a&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'a&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a class='enum' href='http://doc.rust-lang.org/nightly/core/option/enum.Option.html' title='core::option::Option'>Option</a>&lt;&amp;'a <a href='http://doc.rust-lang.org/nightly/std/primitive.str.html'>str</a>&gt;&gt; for <a class='struct' href='git2/struct.StringArrayItems.html' title='git2::StringArrayItems'>StringArrayItems</a>&lt;'a&gt;","<a class='stability Unstable' title='Unstable: just split up for object safety'></a>impl&lt;'a&gt; <a class='trait' href='http://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html' title='core::iter::Iterator'>Iterator</a>&lt;<a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>&amp;'a [</a><a href='http://doc.rust-lang.org/nightly/std/primitive.u8.html'>u8</a><a href='http://doc.rust-lang.org/nightly/std/primitive.slice.html'>]</a>&gt; for <a class='struct' href='git2/struct.StringArrayBytes.html' title='git2::StringArrayBytes'>StringArrayBytes</a>&lt;'a&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
