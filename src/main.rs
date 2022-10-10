use rand::Rng;
use std::io;

fn main(){
    let mut rng = rand::thread_rng();

    /*答え生成*/
    let mut data : [usize;10] = Default::default();
    for i in 1..10{data[i]=i};
    let mut ans : [char;3] = Default::default();
    //非復元抽出
    for i in 0..3{
        let r: usize = rng.gen();
        let num: usize = r %(10-i);
        ans[i] = data[num].to_string().parse().unwrap();

        let tem = data[num];
        data[num] = data[9-i];
        data[9-i] = tem;
    }
    println!("答え: {:?}",ans);

    let mut count = 0;
    loop {
        let mut get = String::new();
        io::stdin().read_line(&mut get).expect("TODO: panic message");
        if(get.len() !=4){ //文字列の最後に終了コードが入ってる！
            println!("3桁で入力してください");
            continue;
        }

        count+=1;
        let mut bingo = (0,0);
        for i in 0..3{
            match get.chars().nth(i){
                Some(v)=>{
                    if ans[i] == v {bingo.0+=1;}
                    else if ans.contains(&v) {bingo.1+=1};
                },
                None => {println!("{}文字目の取得に失敗しました",i+1)}
            }

        }
        println!("HIT：{}, BLOW:{}",bingo.0,bingo.1);
        if bingo.0 == 3 {break;}
    }
    println!("挑戦回数: {}",count);
}