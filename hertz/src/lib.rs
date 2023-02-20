use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, AttributeArgs, NestedMeta, LitBool, LitInt};
use quote::{quote, format_ident};

#[proc_macro_attribute]
pub fn hertz_auto(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_auto");
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&SlashBundle<'_>){
            if let Err(why) = #fname(bnd).await{
                println!(" error on autocomplete {}",why);
            }
        }
    };
    quete.into()
}
fn get_bool<'a>(inp:&'a NestedMeta)->&'a LitBool{
    match inp{
        syn::NestedMeta::Lit(x) => match x {
            syn::Lit::Bool(y)=>y,
            _=>panic!("doesnt see bool")
        }
        _=>panic!("doesnt see bool")
    }
}
fn get_cooldown<'a>(inp:&'a NestedMeta)->&'a LitInt{
    match inp{
        syn::NestedMeta::Lit(x) => match x {
            syn::Lit::Int(y)=>y,
            _=>panic!("doesnt see int")
        }
        _=>panic!("doesnt see int")
    }
}
#[proc_macro_attribute]
pub fn hertz_slash_normal(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&SlashBundle<'_>){
            let user = bnd.user();
            let cmd = bnd.cmd();
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
            if let Err(why)=bnd.cooldown(cd).await{
                return why.log(cmd, &on, false, &mut err).await;
            }
            if defer{
                cmd.defer_res(&mut err, &on,false).await;
            }
            if let Err(why) = #fname(bnd).await{
                match !defer{
                    true=>why.log(cmd, &on, false, &mut err).await,
                    false=>why.log_defer(cmd, &on, &mut err).await,
                };
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn hertz_msg(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let admin = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&MsgBundle<'_>){
            let on = "message command".to_string();
            let admin = #admin;
            let mut err = crate::ErrorLog::new(&bnd.ctx,&bnd.init,&bnd.msg.author).await;
            if admin{
                if !bnd.msg.author.has_role(&bnd.ctx.http,bnd.msg.guild_id.unwrap().to_owned(), RoleId(std::num::NonZeroU64::new(bnd.init.server_role.admin_role).unwrap())).await.unwrap(){
                    let er = MyErr::Custom("You dont have previleges to use this command".to_string());
                    return er.log_msg(&bnd.msg,&on,&mut err).await;
                }
            }
            if let Err(why) = #fname(bnd).await{
                println!("got err {}",why);
                why.log_msg(&bnd.msg, &on, &mut err).await;
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn hertz_modal_normal(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&ModalBundle<'_>){
            let user = bnd.user();
            let cmd = bnd.cmd();
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
            if let Err(why)=bnd.cooldown(cd).await{
                return why.log(cmd, &on, true, &mut err).await;
            }
            if defer{
                cmd.defer_res(&mut err, &on,true).await;
            }
            if let Err(why) = #fname(bnd).await{
                match !defer{
                    true=>why.log(cmd, &on, true, &mut err).await,
                    false=>why.log_defer(cmd, &on, &mut err).await,
                };
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn hertz_button_normal(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&ComponentBundle<'_>){
            let user = bnd.user();
            let cmd = bnd.cmd();
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
            if let Err(why)=bnd.cooldown(cd).await{
                return why.log(cmd, &on, true, &mut err).await;
            }
            if defer{
                cmd.defer_res(&mut err, &on,true).await;
            }
            if let Err(why) = #fname(bnd).await{
                match !defer{
                    true=>why.log(cmd, &on, true, &mut err).await,
                    false=>why.log_defer(cmd, &on, &mut err).await,
                };
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn hertz_slash_reg(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name(bnd:&SlashBundle<'_>){
            let user = bnd.user();
            let cmd = bnd.cmd;
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
            if let Err(why)=bnd.cooldown(cd).await{
                return why.log(cmd, &on, false, &mut err).await;
            }
            let reg = match Reg::switch(bnd.ctx(), cmd, bnd.init(), false, false).await{
                Ok(x)=>{
                    match x{
                        Some(y)=>y,
                        None=>{return;}
                    }
                }
                Err(why)=>{
                    return why.log(cmd, &on,false, &mut err).await;
                }
            };
            if defer{
                cmd.defer_res(&mut err, &on,false).await;
            }
            if let Err(why) = #fname(bnd,reg).await{
                match !defer{
                    true=>why.log(cmd, &on,false, &mut err).await,
                    false=>why.log_defer(cmd, &on, &mut err).await,
                };
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn hertz_combine_normal(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name<T:Mybundle>(bnd:&T){
            let user = bnd.user();
            let cmd = bnd.cmd();
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
            if let Err(why)=bnd.cooldown(cd).await{
                return why.log(cmd, &on, true, &mut err).await;
            }
            if defer{
                cmd.defer_res(&mut err, &on,true).await;
            }
            if let Err(why) = #fname(bnd).await{
                match !defer{
                    true=>why.log(cmd, &on, true, &mut err).await,
                    false=>why.log_defer(cmd, &on, &mut err).await,
                };
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn hertz_combine_reg(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as AttributeArgs);
    let mut iter = args.iter();
    let cooldown = get_cooldown(iter.next().unwrap());
    let defer = get_bool(iter.next().unwrap());
    let fname = &item.sig.ident;
    let new_name = format_ident!("discord_{}",fname.to_string());
    let quete = quote!{
        #item
        pub async fn #new_name<T:Mybundle>(bnd:&T){
            let user = bnd.user();
            let cmd = bnd.cmd();
            let on = bnd.name();
            let cd = #cooldown;
            let defer = #defer;
            let mut err = crate::ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
            if let Err(why)=bnd.cooldown(cd).await{
                return why.log(cmd, &on, true, &mut err).await;
            }
            let reg = match Reg::switch(bnd.ctx(), cmd, bnd.init(), false, true).await{
                Ok(x)=>{
                    match x{
                        Some(y)=>y,
                        None=>{return;}
                    }
                }
                Err(why)=>{
                    return why.log(cmd, &on,true, &mut err).await;
                }
            };
            if defer{
                cmd.defer_res(&mut err, &on,true).await;
            }
            if let Err(why) = #fname(bnd,reg).await{
                match !defer{
                    true=>why.log(cmd, &on,true, &mut err).await,
                    false=>why.log_defer(cmd, &on, &mut err).await,
                };
            }
        }
    };
    quete.into()
}
#[proc_macro_attribute]
pub fn test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let idk = parse_macro_input!(input as ItemFn);
    let qt = quote!{
        #idk
    };
    qt.into()
}
