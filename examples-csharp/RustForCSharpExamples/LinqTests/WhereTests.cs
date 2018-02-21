using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using NUnit.Framework;

namespace LinqTests
{
    [TestFixture]
    public class WhereTests
    {
        [Test]
        public void Where1_Ints()
        {
            var source = new List<int> { 1, 2, 3, 4 };
            var result = source.Where(x => x > 2).ToList();
            CollectionAssert.AreEqual(result, new List<int> { 3, 4 });
        }

        [Test]
        public void Where1_Strings()
        {
            var source = new List<string> { "red", "green", "blue", "white", "yellow" };
            var result = source.Where(x => x.Contains("w")).ToList();
            CollectionAssert.AreEqual(result, new List<String> { "white", "yellow" });
        }
    }
}
