
use std::alloc::System;
use std::io::{Write, Read};
use std::str::FromStr;
use regex::{Regex, Captures};
use proc_macro::{TokenStream};
use quote::ToTokens;
use syn::parse::{ParseStream, Parse};
use syn::punctuated::Punctuated;
use syn::{DeriveInput, parse_macro_input, token, Field, Error, Ident};

extern crate proc_macro;

struct Komio_Fn_Param
{
    m_Name:String,
    m_Type:String
}
#[proc_macro]
pub fn Did_Gen_Start( item:TokenStream)->TokenStream{
    Clear_Temp_File();
    return TokenStream::new(); 
}
#[proc_macro]
pub fn Did_Gen_End( item:TokenStream)->TokenStream{
    let mut t_path=item.to_string();
    t_path=t_path.replace("\"", "");
    print!("\n@PATH:{}\n",t_path);
    Save_To_Json(t_path);
    return TokenStream::new(); 
}
// 属性宏 （两个参数）
#[proc_macro_attribute]
pub fn Did_Gen(attr:TokenStream, item:TokenStream)->TokenStream{
    let t_attr_str=attr.to_string();

    let t_item=item.to_string();

    let t_stream=TokenStream::from_str(&t_item).unwrap();

    let t_parse_result:Result<ItemStruct,Error>=syn::parse_macro_input::parse::<ItemStruct>(t_stream.clone());
    if(t_parse_result.is_ok())
    {
        
        println!("Fields:{}\n",t_parse_result.unwrap().fields.into_token_stream().to_string());
    }
    let mut  t_iter=t_stream.into_iter();

    let mut t_token_item=t_iter.next();

    let mut t_token_vec:Vec<String>=Vec::new();

    while(t_token_item.is_some())
    {

        let mut t_item_unwrap=t_token_item.unwrap();
        
        //println!("Token:{}\n",t_item_unwrap.to_string());
        t_token_vec.push(String::from( t_item_unwrap.to_string()) );
        t_token_item=t_iter.next();

    }
    for i in 0..t_token_vec.len()
    {
        println!("Token:{}\n",t_token_vec[i].to_string());
    }

    let mut t_fn_token_index:i32=-1;
    let mut t_return_token_index:i32=-1;
    for i in 0..t_token_vec.len()
    {
        if(t_token_vec[i].eq("fn"))
        {
            t_fn_token_index=i as i32;
        }
    }
    println!("fn:{}\n",t_fn_token_index);

    if(t_fn_token_index>=0)
    {
        let mut t_fn_name=t_token_vec .get((t_fn_token_index as usize)+1).unwrap().clone();
        let mut t_params=t_token_vec .get((t_fn_token_index as usize)+2).unwrap().clone();
        t_params= t_params.replace("(", "");
        t_params= t_params.replace(")", "");
        t_params= t_params.replace(" ", "");
        let mut t_param_slice=t_params.split(",");
        let mut t_params_vec:Vec<Komio_Fn_Param>=Vec::new();
        let mut t_type_nat_regex = Regex::new(r"(u8)|(u16)|(u32)|(u64)|(u128)|(i8)|(i16)|(i32)|(i64)|(i128)|usize|isize").unwrap();
        let mut t_type_text_regex = Regex::new(r"(String)|(str)").unwrap();
        let mut t_type_vec_regex = Regex::new(r"Vec\x3c(([a-z0-9]|Nat)*?)\x3e").unwrap();
        for item in t_param_slice {

            let mut params= item.split(":");
            let mut t_word=params.next().unwrap();
            let mut t_word_type=params.next().unwrap();
            
            let mut t_word_type_new=t_type_nat_regex.replace(&t_word_type, r"Nat");
            t_word_type=t_word_type_new.as_ref().clone();

            
            let mut t_word_type_new=t_type_text_regex.replace(&t_word_type, r"Text");
            t_word_type=t_word_type_new.as_ref().clone();

            
            let mut t_word_type_new=t_type_vec_regex.replace(&t_word_type, r"[$1]");
            t_word_type=t_word_type_new.as_ref().clone();
            println!("fn_param_type:{}\n",t_word_type);
            t_params_vec.push(Komio_Fn_Param { m_Name: String::from_str( t_word).unwrap(), m_Type: String::from_str(t_word_type).unwrap() })
        };
///////////////////////////////////////////////////////////////////////////////////////////////////////////////
        let mut t_return_type=t_token_vec .get((t_fn_token_index as usize)+5).unwrap().clone();
        let mut t_word_type_new=t_type_nat_regex.replace(&t_return_type, r"Nat");
        t_return_type= String::from( t_word_type_new.as_ref().clone());
        let mut t_word_type_new=t_type_text_regex.replace(&t_return_type, r"Text");
        t_return_type= String::from( t_word_type_new.as_ref().clone());        
        let mut t_word_type_new=t_type_vec_regex.replace(&t_return_type, r"[$1]");
        t_return_type= String::from( t_word_type_new.as_ref().clone());
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////

        println!("fn_return_type:{}\n",t_return_type);
        Save_To_Temp_File(&t_fn_name,&t_params_vec,&t_return_type,&t_attr_str);
        println!("fn_params_count:{}\n",t_params_vec.len());
    }
    

    //let derive_input =parse_macro_input!(t_stream as syn::DeriveInput);
    //println!("Attr:{}", attr.to_string());
    //println!("S:{}", derive_input.ident);
    println!("Item:{}", t_item.to_string());
    return TokenStream::from_str(&t_item).unwrap();
}
fn Clear_Temp_File()
{
    let mut  t_file_option= std::fs::OpenOptions::new();
    t_file_option.write(true);
    t_file_option.create(true);
    t_file_option.truncate(true);
    let mut t_file= t_file_option.open("./komio_macro_temp.txt").unwrap();
    t_file.flush().unwrap();
}
fn Save_To_Json(path:String)
{
    let mut  t_file_option= std::fs::OpenOptions::new();
    t_file_option.create(true);   
    t_file_option.read(true); 
    t_file_option.truncate(false);
    t_file_option.write(true);
    print!("\n@0\n");
    let mut t_file= t_file_option.open("./komio_macro_temp.txt").unwrap();
    let mut t_file_string=String::new();
    print!("\n@1\n");
    t_file.read_to_string(&mut t_file_string).unwrap();
     t_file.flush().unwrap();


    let mut  t_json_option= std::fs::OpenOptions::new();
    t_json_option.create(true);    
    t_json_option.truncate(true);
    t_json_option.write(true);
    t_file_option.append(true);
    print!("\n@2\n");
    let mut t_json_file= t_json_option.open(path).unwrap();
    print!("\n@3\n");
    t_json_file.write(String::from("service : {\n").as_bytes()).unwrap();
    t_json_file.write(t_file_string.as_bytes()).unwrap();
    t_json_file.write(String::from("}").as_bytes()).unwrap();
    t_json_file.flush().unwrap();
}
fn Save_To_Temp_File(f_name:&String,f_param:&Vec<Komio_Fn_Param>,return_type:&String,modifier:&String)
{

   let mut  t_file_option= std::fs::OpenOptions::new();
   t_file_option.create(true);
   t_file_option.append(true);
   let mut t_file= t_file_option.open("./komio_macro_temp.txt").unwrap();
   let mut t_str:String =String::new();
   t_str+="\"";
   t_str+= &f_name;
   t_str+="\":(";
   
   for item in f_param
   {
        t_str+=&(item.m_Type);
        t_str+=",";
   }
   t_str.remove(&t_str.len()-1);
   t_str+=")->(";
   t_str+=&return_type;
   t_str+=") ";
   t_str+=&modifier;
   t_str+=";\n";
   print!("SAVE:{}\n",t_str);
   t_file.write( &mut t_str.as_bytes()).unwrap();


   t_file.flush();
   
   // ::open("./komio_macro_temp.txt").unwrap();
  //t_file.w
}


struct ItemStruct {
    struct_token: ::syn::token::Struct,
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, ::syn::token::Comma>,
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self,Error> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: match ::syn::group::parse_braces(&input) {
                ::syn::__private::Ok(braces) => {
                    content = braces.content;
                    braces.token
                }
                ::syn::__private::Err(error) => {
                    return ::syn::__private::Err(error);
                }
            },
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}