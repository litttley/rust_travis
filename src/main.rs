extern crate reqwest;
extern crate select;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class,Name};
//use std::io::{Read};
use std::fs::File;
use std::fs::create_dir_all;
use std::io::{BufWriter, Write};
use std::io;
//use json::JsonValue;
fn main() {
    let mut start_url = String::new();
    println!("请输入键接(如https://blog.csdn.net/m0_37696900/)");


    io::stdin().read_line(&mut start_url).expect("输入错误");

    let user_name =   start_url.replace("https://blog.csdn.net/","").replace("/","").replace("\r\n","").replace("\n","");
    println!("user_name{:?}",user_name);
   // let mut  resp = reqwest::get("https://blog.csdn.net/m0_37696990/").unwrap();
    let mut  resp = reqwest::get(start_url.trim()).unwrap();
    let   html_body = resp.text_with_charset("utf-8").unwrap();
    let document = Document::from( html_body.trim());
   /* let s =  include_str!("stackoverflow.html");*/

    for node in document.find(Attr("id","asideCategory").descendant(Name("a"))) {
           // let  _value = node.text().replace("\n","");
            let  mut moudle_name;
            if  let  Some(t) = node.find(Class("oneline")).next(){
                moudle_name = t.text();
            }else{
             //   moudle_name="".to_string();
                continue;
            }
            println!("模块名称{:?}",moudle_name);
        let mut count_string ;
        let   num_count;
        let   page:i32;
        if let Some(t) = node.find(Class("count")).next(){
               count_string = t.text().replace("篇","");
            //println!("count_string:{:?}",count_string);
                num_count=count_string.parse::<i32>().unwrap();
           // println!("num_count{}",num_count);
                page =  num_count/20;
        }else{
            continue;
        }

        let  mut  value_href=String::from("") ;
            if let Some(t)  =  node.attr("href"){
               value_href.push_str(t);
            }else{
                continue;
            }
        if page > 0 && num_count >0{
            println!("分页");
            let   value_href_new = value_href.clone();
            if  num_count % 20 == 0{
                println!("偶数");
                for node_page in 0..page{

                    //value_href_new.push_str("/");
                    let page_int = node_page+1;
                   // value_href_new.push_str(&page_int.to_string());
                   // value_href_new.push_str("?");
                    let  value_href =   format!("{0}{1}{2}{3}",value_href_new,"/",page_int,"?");
                    println!("value_href_for:{:?}",value_href);
                    get_moudle_list(value_href.clone(),moudle_name.clone(),user_name.clone());
                }
            }else{
                for node_page in 0..page+1{
                    println!("奇数");
                    let page_int = node_page+1;
                  let  value_href =   format!("{0}{1}{2}{3}",value_href_new,"/",page_int,"?");
                  //  value_href_new.push_str("/");

                    //value_href_new.push_str(&page_int.to_string());
                   // value_href_new.push_str("?");
                    println!("value_href_for:{:?}",value_href);
                    get_moudle_list(value_href.clone(),moudle_name.clone(),user_name.clone());
                }
            }


        }else{
            println!("不分页");
            //println!("{:?}{:?}",value,&value_href);
            get_moudle_list(value_href,moudle_name.clone(),user_name.clone());
        }




    }

    let mut end = String::new();
    println!("下载完成：");
    io::stdin().read_line(&mut end).expect("输入错误");



}


fn get_moudle_list(url:String,moudle_name:String,user_name:String){
    let mut  resp = reqwest::get(url.trim()).unwrap();
    let   html_body = resp.text_with_charset("utf-8").unwrap();
    let document = Document::from( html_body.trim());
   // let mut path = String::from("./blogs/");
   // path.push_str(&moudle_name);
     //create_dir_all(&path).expect("创建目录失败");
    //println!("{:?}",html_body);
    for node in document.find(Class("article-item-box").descendant(Name("a"))) {
        let  mut  value_href=String::from("") ;
        if let Some(t)  =  node.attr("href"){
            value_href.push_str(t);
        }else{
            continue;
        }
      //  println!("{:?}",&value_href);
        if value_href.contains(&user_name){
            get_detial(value_href,moudle_name.clone());
        }

    }
    //println!("退2");
}


fn get_detial(url:String,moudle_name:String){
    let mut  resp = reqwest::get(url.trim()).unwrap();
    let   html_body = resp.text_with_charset("utf-8").unwrap();
    let document = Document::from( html_body.trim());
    for node in document.find(Class("blog-content-box")) {
        let value_title = node.find(Class("title-article")).next().unwrap().text();
       // println!("value_title{:?}",value_title);
        let  value_title =  value_title.replace("/","").replace("*","").replace(":","").replace(">","").replace("<","");
        let  mut  value_conten=  node.find(Attr("id", "article_content")).next().unwrap().html().clone();
        let mut path = String::from("./blogs/");
        path.push_str(&moudle_name);
        path.push_str("/");
        path.push_str(&value_title);
        let mut  path2 = path.clone();
        path.push_str("/images");
        //path.replace("/","").replace("*","").replace(":","");
       // println!("目录path{:?}",path);

        create_dir_all(&path).expect("创建目录失败");

        for img in   node.find(Attr("id", "article_content").descendant(Name("img"))){
           let mut img_href=String::from("");
            if let Some(t)  =  img.attr("src"){
                img_href.push_str(t);
               let image_path =  get_img(img_href.to_string(),path.clone());
                if image_path.is_empty(){
                    continue;
                }
                value_conten = value_conten.replace(&img_href,&image_path);
           //     println!("image_path{:?}",image_path);
               // let mut  resp = reqwest::get(img_href.trim()).unwrap();
            }else{
                continue;
            }
         //   println!("img_url:{:?}",img_href);
        }
        path2.push_str("/");
        path2.push_str(&value_title);
        path2.push_str(".html");
        match  File::create(path2){
            Ok(t) =>{
                let mut f = BufWriter::new(t);
                f.write_all(value_conten.as_bytes()).expect("写入文件失败");
                println!("文件保存成功");
            },
            Err(e) =>println!("图片保存失败{:?}",e),
        };

        //println!("{:?}",value_title);

    }

   // println!("退1");

}

fn get_img(url:String,  path:String) -> String{

    println!("加载图片1{:?}",url);

    let imgae_name_arr:Vec<&str> =url.split("?").collect();
  //  println!("imgae_name_arr{:?}",imgae_name_arr);
    let image_name=  imgae_name_arr.get(0).unwrap().to_string();
    let image_name_end:Vec<&str>  =   image_name.split("/").collect();
    let  image_name_end_str  =   image_name_end.last().unwrap().to_string();
  //  println!("imgae_name{:?}",image_name_end_str);
    let    _bytes_arr:Vec<u8> = Vec::new();
    let mut  resp ;
    if let Ok(t) =  reqwest::get(url.trim()){
        resp = t;
    }else{
        println!("图片加载失败");
        return  "".to_string();
    }

    //let s =  resp.text().unwrap();
    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf).unwrap();
    let mut  path = path.clone();
    path.push_str("/");
    path.push_str(&image_name_end_str);
    path.push_str(".png");



    match  File::create(&path){
        Ok(t) =>{
            let mut f = BufWriter::new(t);
            f.write_all(buf.as_slice()).expect("写入文件失败");
            println!("图片保存成功");
        },
        Err(e) =>println!("图片保存失败{:?}",e),
    }

     let  mut  return_path = String::from("./images/");
    return_path.push_str(&image_name_end_str);
    return_path.push_str(".png");
 return    return_path ;
  // println!("msg:{:?}",msg);

}