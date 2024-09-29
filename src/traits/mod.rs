mod convertible_to_mask;
pub use convertible_to_mask::*;

mod find_unique_sets;
pub use find_unique_sets::*;

mod implementations;

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    mod convertible_to_mask {
        use super::*;

        macro_rules! create_u32_test {
            ($name:ident($char:literal=>$expected:expr)) => {
                #[test]
                fn $name() {
                    let mask: u32 = $char.to_mask();
                    assert_eq!(mask, $expected);
                }
            };
        }
        // `a` needs to be 1 not 0 because any mask is 1-indexed to indicate if `a` is present.
        create_u32_test!(mask_u32_a('a' => 1));
        create_u32_test!(mask_u32_b('b' => 2));
        create_u32_test!(mask_u32_c('c' => 4));
        create_u32_test!(mask_u32_d('d' => 8));
        create_u32_test!(mask_u32_e('z' => 1<<25));
        // `A` will overlap with `a` because `u32` only supports 32 unique masks.
        create_u32_test!(mask_u32_A('A' => 1));

        macro_rules! create_u64_test {
            ($name:ident($char:literal=>$expected:expr)) => {
                #[test]
                fn $name() {
                    let mask: u64 = $char.to_mask();
                    assert_eq!(mask, $expected);
                }
            };
        }
        // `a` needs to be 1 not 0 because any mask is 1-indexed to indicate if `a` is present.
        create_u64_test!(mask_u64_A('A' => 1));
        create_u64_test!(mask_u64_a('a' => 1 << 26));
    }

    mod find_first_unique_set_index {
        use super::*;

        macro_rules! create_test {
            ($name:ident::<$size:literal>($string:literal) => $expected:expr) => {
                #[test]
                fn $name() {
                    let result = $string.chars().find_first_unique_set_index::<$size, u32>();
                    assert_eq!(result, $expected);
                }
            };
        }

        create_test!(simple_test::<3>("abcd") => Some(0));
    }

    macro_rules! create_test {
        ($name:ident($string:expr)) => {
            #[test]
            /// Test the optimized implementation against the naive implementation.
            fn $name() {
                let string = $string;
                let result = string.chars().find_first_unique_set_index::<14, u32>();
                let result_naive = string.chars().find_first_unique_set_index_naive::<14>();

                assert_eq!(result, result_naive);

                const COUNT: usize = 1000;
                let timed = string.chars().time_first_unique_set_index::<14, u32>(COUNT);
                let timed_naive = string
                    .chars()
                    .time_first_unique_set_index_naive::<14>(COUNT);

                println!("Found: {:?}", result);
                println!("Optimized: {} µs", timed);
                println!("Naive: {} µs", timed_naive);
                println!("Optimized is {} times faster", timed_naive / timed);

                // The optimized implementation should be at least 1.5 times faster than the naive implementation.
                assert!(timed_naive / timed > 1.5)
            }
        };
    }

    const LONG_TEXT_NO_MATCH: &str = "aaggbblfgmngkajfciafgailabgnfncfbkcgnndfijmlckeckjfghcicicmaaabnlhdjhlgccibndfdiinljigmfnfefidckkmneddindncnjdagabalclfkgbkjmmhgicfkbmlkmhkcddelkdlhikllicjhcamllbmemgkmhfcjkmeileebkhklmhchifcihfkcabflifficflcdcclbjgnehadghlkhlkaigaaclakcmdiilhnlfinnjegbjmalkknfddecbfdgcjlffhmlalkfanbieckikailaecachckjidgefikemfhladnjhdngicfdlgmejdmighambnhjgkmiifidekdbgleejeejalilkjfhneijhagcjdldbbegiiihhkgmjmedlhljjlnegagnjcecdnjeaklnmcllaciicmifglgekjffhdmafbkilcbmbkanfjnmfgghgicbfdenllmmkebmdkenbkjbmfakgcjlkdhgjkckhggbnhdbafjmmibnhllkkgnmnjidjeleaihljklmknabhadmfeebgjjfhdmcbcahnfdnhfeackaknnfldlkecnlhmgmlihlklnfnckmgghcblmjgaijmaefiehbfdgchjcjkajablkefjeilhinkiegkcebbfgcjfeglmaljcgmllmeddhikeimdlngfdallhgjnhenbkjkmfbcfneeabfakjceiliigfinhjfkidbbfknkmiekfdflimekjnfcfidindkmlmkkmhjfjkiaamdnnfbginmdfmmgcfiabnhbbibnlmjlledbamhbnaidgjinfiamadeaikhnaakbihlhnnklkhbjdadabjgdgghcmijenkmhglgefmkffgkkjebachabkhlhjffkickhchdikfdjgkjlbajdjibccikcgebbjdkflimkcenjeelekleidkclkaiabgajicnfnenlcccflagmcijbfjibgblhhgjkjficbkikmahiihhibhlnmjlakbikhfbchabmlmciffjbcmndljhcmmgefceccfdffaahnkjiiahddnigmnabgeeaeegmnbehmfbkkkcikfeeanlenehfnbgckjggdemficajgifjlnldlhbefccmeelgliffjhcelkmjdhbnfbbfccldanljmfkajmjkllghhahhdjjmnknjhmehkgblllijbamhjehfcngbmjekjlfehnjihlfjmkiaakifahagmgdaainfnbmbgmkffbmlfincbcbebbenabnimlagabaidcbhkijelbfnlnbfneglbjcckemijmhnmldjlgkbkalfkkakhjeiglafickigklafdklilkjkfgkkiaimhnhaacjdmmcnjcdfhlngmmgagcddaijddfgdackmdkdcbbkdfbgahmfkingaclgchknkcjggffbcbhkekchjjdaihbnkfilfgkhjcnecjejjgdeheikjhddegljhgigcdlaljbjjbddlclmgkkgjkhaiadenaljcadckhifffieaingmkhgadmanmjmhfgmnlglinbglfeebcihblhcgdgnmicaaeleaaahfmnnglbncckbghcefbnhllfekdcmdmjblmekceeeidjbeifaagecidlmlgemendenemglebkfdglgdlhnebcmgaignainhjffcdgecgjelmjhlnlfddcmhnjbkccjffabbdnhankbkfmnhjcgibabkjlieeakdcklenhkialfnihjnjjameeknmgdnifnhakmcfinmjbekkdngbbcnncmcfkiaabghgnhdikajkkaifglhjedgllalghimmmbhmhncekmdfigdamgjcbdclgjficgmcdimdkkclihheekcebbldldggkejdngachlmnhdednfngcinbglefncdkgjclcaagfkbajabmdbddlbllddcjfjngfdagajidaffcnnjkhbbjfbamgfneafhnacklajfmjlmjedkmabjfechbjbimckcfiiamllgfmmdmkhlhakcmlkhlgbjknmagbacemjhkeigncjeebjalnefkmgeecagniecakngcfeenhhefginlhindimnijgieljgeeehjkafjlnmdblbehmhialbchijehblmalkedjdmbfadhhmhffdfgcndalmknandbbcbanhkjdekicdegkajdadjfikknhcfahgngeechlekmnkkfdglemebhdfelinbcjfbmjkecijmfmccmjllmknafgkfidgdbicbbegbhbgancnbkigijafmefkaeilngdcncmjfffjghlffncfciemblibcmkegkeaeehclmimmnedhngdhljigjjnhhfafcgnnlijhcegmdjfgjkfbmmfejlmcbiblehmmamliibliablfbidfmmehmfcgdbbdcnlfdnmflncgmbfjnndligeblakfadeembiidklcajjblhidikggbdnkelfkababcccicdfnehgljmmflimeggkkjgccgfekehkmhdemlnaifbaaicchnhlkflmfienjfincgkejnjacggmgenlekccfnmhcjladhbmbfaekfcabddfjamejjcjjihjndehkmmiabefcmkjkaglfcjhlijjgbmaamacbfmmcmgdfdggggmncechgfjlhjjminklcegicnighgfkdiljielcmknlmhcbinhcakfhhldmadjffjiabaihdlkmndehankmneibieimmlmdaiknhfmfacjggannmjhcbjjfgaihecbafknegajecgnbejfkdcgmbhifjenbiekijbmciebcgegjhkaghjkehgfbfjffhlhljhadcckcnlbinchjblaenhkedmchefkjmeggihmcfgchbckbcjhbamblgijnbjjldgjmhbeaglmhjalgdmmfehackkmahhnbaiediackdnbdnaajjhckjmeaeilnddmhjhefnnabncicnmdlceedebkmnlggdjdjgimfkedbnlijalmakailemnffnalahgmjmbljkhmejjgdcebmnfbmdnjkmgaanihggfgmheiegicmhhbljamgddgfkgjimneemieabiilmlhmginggdhdlljkmfkjbmfgbbhjkkikikjdlbjnlimigmbagjjeheedmdcfhfgjmfeagjfbilikfcniidkbaebmhgdaeaggeahalcgilgdjmmjhcflmifncmiaemkgibajccnkjfbdneckdnmcbbadedkflmkmdhnkbdckninimijlhcfjbjggnkjakegdkiejlgieganjlbjbjhckmmfkljcjehehiaadkjjchnlcikghmkkfbigikljkbbmdflneecnefcgmmjiigknakcfmhfnebemgnfggjemmcdhieceeikaakmegkgdaimfnhmfdkdclkdmlijcklhenkaigjibhggalimjnehkadgbndbheianakmlahnebcealdaecnjmechliiikdchegahcihmjjifghjibnflbffjkebgikajfkmhemiedeecldeidkedkicbfaekbhbmhllachbiehckbajjdgjhdklknbijegnaiabglmglbcdbdhhhmcnglbimmbknghikehagmnefnmmdhbcbfbglijaalimjngbbaalmdakejlmhlcfhlldnbbgbbmmicgjibkmnfgambkkigicilgndaiffbhengfcanfimccmkcilncanblelkiafganmfkedigcccaancjglkgebglhkachcghlcanjgbcbialhjcikgdifmdjaaklbjhhlhenhedbngdmiankbmkjbeefmenfiejmiieknldgecifklefgdkfnleefclcemimjjkmeiadbdacnenlkanllhajegcaanalbnbgknelcagjefblffcagkjbhdmfjjlaacicnfjdfgnngnacnfnhjefnlibkhlcmljigghbdjldaiafdfhkilclnfgfkkgegcamfefdcdibfgfmhfengcmmfhebejafajm";
    create_test!(leet_code_example("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    create_test!(long_text_no_results(LONG_TEXT_NO_MATCH));
    create_test!(long_text_results_at_rear(
        LONG_TEXT_NO_MATCH.to_owned() + "nopqrstuvxyzabcde"
    ));
}
