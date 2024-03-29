// 2439. Minimize Maximum of Array
// https://leetcode.com/problems/minimize-maximum-of-array/

use crate::Solution;

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        // we need to find max ceil average
        // [10] => 10
        // [10, 12] => 11
        // [10, 11] => 11
        // [10, 11, 11] => 11
        // [10, 11, 11, 11] => 11
        nums.iter()
            .zip(0..)
            .fold((i64::MIN, 0), |(res, mut sum), (&x, i)| {
                sum += x as i64;
                let avg = (sum + i) / (i + 1); // variant of ceil average with integer (math trick)
                (res.max(avg), sum)
            })
            .0 as _
    }

    // https://leetcode.com/problems/minimize-maximum-of-array/discuss/2707159/Rust-11-ms-fastest-(100)-one-liner-%2B-PROOF-(with-detailed-comments)
    pub fn minimize_array_value_map(nums: Vec<i32>) -> i32 {
        // here is the proof why it works...
        // [a] the flow of value is only to the left;
        // [b] this eventually leads to a non-increasing list 'final';
        // [c] in the end, the element 'final[0]' will contain the answer;
        // [d] however, to detect this element without simulating the actual flow,
        //     we notice that, for each prefix, its average cumulative sum can
        //     not surpass the 0th element of the 'final' list (pigeonhole principle)
        // [e] the question is, will the ceiled value of the average prefix sum
        //     ever be equal to the initial element of the resulting list (for some prefix)?
        // [f] yes, because if for ALL prefixes their average cumsum < 'final[0]'
        //     then it is simply not possible to transfer value from right to left
        //     such that it becomes 'final[0]' (can be proved by induction on prefixes)
        let mut sum: usize = 0;
        nums.into_iter()
            .enumerate()
            .map(|(i, n)| {
                sum += n as usize;
                (sum + i) / (i + 1)
            })
            .max()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::minimize_array_value(vec![3, 7, 1, 6]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::minimize_array_value(vec![10, 1]), 10);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::minimize_array_value(vec![
                153096409, 343881218, 741913853, 343594218, 864722890, 354938680, 279386271,
                616365038, 896106991, 540459582, 124304477, 856321779, 533947835, 590383040,
                708653960, 928865842, 501462358, 113265076, 786991139, 83872665, 314304738,
                719655858, 685019739, 773289565, 224287062, 961183249, 922185605, 437586814,
                431957201, 622418600, 246298056, 991189969, 411305056, 270647246, 245431042,
                698124847, 829853159, 270399508, 670957159, 333775594, 8772554, 816887664,
                443241521, 730170733, 221327413, 52643856, 245385722, 79672837, 350678160,
                899468390, 969547281, 790832565, 240362170, 335105823, 56655569, 352858708,
                282269718, 803703253, 304986773, 102709585, 615892285, 759458532, 699157897,
                706378502, 217026516, 963906270, 328782137, 832395397, 801082006, 736947500,
                740380649, 320003089, 442712291, 921177417, 319837024, 864463340, 925725724,
                623163350, 720854915, 677738988, 571962532, 208002838, 811990072, 857491066,
                330102375, 483610885, 857860635, 923028353, 365219532, 123612310, 358057577,
                556762745, 276409227, 379045020, 726246458, 694876970, 572529495, 45766766,
                4543357, 962003438, 71963639, 353396710, 471693853, 238090807, 580186750,
                888354824, 575610184, 834405856, 511625103, 507217807, 758041749, 752190512,
                94095006, 308735380, 158529578, 90454388, 211426730, 649697704, 744118303,
                836898375, 40356689, 870938450, 284175275, 680099256, 81201202, 725528383,
                395833008, 215087720, 230063778, 87890347, 272154478, 444833325, 369153746,
                946233671, 363564600, 575689591, 274962806, 90775002, 761470515, 290820736,
                722148657, 43643603, 97377877, 24124498, 661445223, 187999299, 752711, 449250032,
                597874808, 446918491, 208817895, 906371979, 327494570, 75366573, 554239448,
                814087984, 657085854, 270692874, 52218799, 382938142, 412832746, 672415037,
                252108701, 171151227, 710909171, 621892259, 115818943, 750481578, 887252051,
                136110429, 947040508, 881172510, 684078208, 11353138, 554153585, 512413931,
                41569924, 802301655, 615268114, 629889281, 127158156, 35472792, 562057054,
                988403333, 183459766, 55449148, 66469418, 685358479, 247296038, 823731835,
                711961003, 77017085, 467728586, 987276303, 465700259, 25542928, 712637026,
                757269491, 280358406, 541929905, 524827919, 357012915, 456996863, 17470382,
                409194182, 291011584, 507963802, 390939068, 538209842, 258156497, 287888887,
                464601073, 512078280, 195569494, 903699053, 998782648, 958479486, 807701514,
                475408653, 584849291, 194776931, 570064933, 289391293, 493732232, 947383428,
                707886584, 790308898, 162235515, 904713864, 522390763, 664867040, 956190643,
                117006139, 813099597, 783312139, 311955688, 330090537, 241496936, 98582723,
                195651756, 56579761, 711047247, 780214711, 455185579, 274172333, 562197999,
                829665305, 307074833, 956116385, 550031657, 396154727, 46661133, 395455564,
                655895260, 408367926, 336474975, 523622945, 649364375, 633026525, 630264148,
                78566520, 195661443, 123240531, 292626379, 610511107, 561507257, 895293142,
                685771635, 701111222, 289065168, 171851738, 101146684, 717958525, 29712102,
                216167164, 572262445, 517878878, 319590424, 530009929, 949513193, 831610251,
                107906093, 417341370, 968876842, 22347664, 764108177, 11944435, 368277846,
                340786349, 898715898, 474256519, 339752101, 633913590, 901380327, 351199901,
                798649839, 584479980, 781687079, 98841219, 56840714, 452749252, 89521588,
                191142445, 680534934, 729356464, 148380187, 524348668, 830471203, 935540006,
                259561503, 823849524, 486867453, 651999440, 730644880, 516164287, 615781121,
                549581800, 146974728, 382084733, 629086971, 135042221, 196781149, 43934850,
                947283406, 673133800, 708896547, 329456772, 757629247, 406750837
            ]),
            554808881
        );
    }
}
