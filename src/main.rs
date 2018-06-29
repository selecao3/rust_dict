extern crate reqwest;
extern crate scraper;

use reqwest::get;
use scraper::Html;
use scraper::Selector;

//static COUNT:i32 = 0;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();


    let url = get(format!("http://public.dejizo.jp/NetDicV09.asmx/SearchDicItemLite?\
    Dic=EJdict&\
    Word={}&\
    Scope=HEADWORD&\
    Match=CONTAIN&\
    Merge=AND&\
    Prof=XHTML&\
    PageSize=20&\
    PageIndex=0",s).as_str())
        .unwrap()
        .text()
        .unwrap();
    let hoge = url.as_str();
    let frag= Html::parse_fragment(hoge);
    let ul_selector = Selector::parse("ItemID").unwrap();
    for element in frag.select(&ul_selector){
        //let hage = element.value().name();
        let hage = element.inner_html();
        let url = get(format!("http://public.dejizo.jp/NetDicV09.asmx/GetDicItemLite?\
        Dic=EJdict&\
        Item={}&\
        Loc=&\
        Prof=XHTML",hage).as_str())
            .unwrap()
            .text()
            .unwrap();
        //println!("{}",url);
        let url_str = url.as_str();

        let frag2 = Html::parse_fragment(url_str);
        let selector = Selector::parse("div").unwrap();

        let mut count:i32 = 0;
        for i in frag2.select(&selector).filter(|_x|{
            count += 1;
            count% 3 == 0
        }){

            //let count = count + 1;
            //カウントできねぇ

            let aaa = i.inner_html();
            println!("{}\n",aaa);
        }

    }
    /*
    println!("================================");
    let url = get("http://public.dejizo.jp/NetDicV09.asmx/GetDicItemLite?Dic=EJdict&Item=011257&Loc=&Prof=XHTML")
        .unwrap()
        .text()
        .unwrap();
    println!("{}",url);
    */
}
