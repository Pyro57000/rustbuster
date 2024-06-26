use std::fs;
use reqwest::{self, StatusCode};
use clap::Parser;
use std::thread;
use dns_lookup::lookup_host;


/// Subdomain Bruteforcer, and directory bruteforcer written in rust, and multi threaded!!!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    /// target to attack, can be a URL or a dns name, you can specify more than one by comma separating the targets
    #[arg(short, long)]
    target: String,

    ///Threads to use, there is not upper limit so if you lock up your comptuer that's on you
    #[arg(long)]
    threads: usize,

    /// wordlist to use for subdomains
    #[arg(short, long, default_value_t = String::from("none"))]
    subwordlist: String,

    ///wordlist to use for directories 
    #[arg(short, long, default_value_t = String::from("none"))]
    dirwordlist: String,
}


fn try_sub(domain: String){
    let ips = lookup_host(&domain);
    if ips.is_ok(){
        let mut ip_string = String::new();
        for ip in ips.unwrap(){
            ip_string = format!("{},{}",ip, ip_string);
        }
        println!("{} {}", domain, ip_string);
    }
}


fn try_dir(url: String){
    let resp_stat = reqwest::blocking::get(&url);
    if resp_stat.is_ok(){
        let resp = resp_stat.unwrap().status();
        match resp{
            StatusCode::OK => println!("{} {}",resp, url),
            StatusCode::ACCEPTED => println!("{} {}", resp, url),
            StatusCode::CONTINUE => println!("{} {}", resp, url),
            StatusCode::CREATED => println!("{} {}", resp, url),
            StatusCode::FOUND => println!("{} {}", resp, url),
            StatusCode::IM_A_TEAPOT => println!("And what a beautiful teapot you are {}", url),
            StatusCode::MOVED_PERMANENTLY => println!("{} {}", resp, url),
            StatusCode::PERMANENT_REDIRECT => println!("{} {}", resp, url),
            StatusCode::TEMPORARY_REDIRECT => println!("{} {}", resp, url),
            _ => ()
        }
    }
}

fn main() {
    print!("
 

 

                                                           ▄

                                                        ╓╦▀

                                                      ╓█▀

                                                     ▀▀

 

                                                              ╓▓

                                            ╓▄              ▄█▀

                                          ▄█▀▓▌           ▄█▀            ▄▀

                                        ▄█╙   █          ▀╙           ╓█▀

                                      ▄▀      █▄   ▄▄▄              ╓█▀

                                     ▐▌       ▐█ ▄▀└ ▐█             └

                                     ▐▌       ╓█▀   ▄█┘

                                     ╟▌     ▄█▀   ▄█▌

                                  ┌▄▄██╖  ▄█▀  ╓▄▀  └╙▀▀▀▀▓▄▄

                              ▄▄▓▀╙    ▀██╙  ╓▓▀           ▄▀

                          ╓▄▓▀└          ╙█▄█▀          ╓█▀

                       ▄██▌                 ▀▄        ▄█▀

                    ▄█▀   ▀▌                 ▐█▀▀▀▀▀▀▀╘

                 ▄█▀ ▀▌    ╙█▄              ╓█

               ▄▀╙    ╙█┐    ▀▓╖           ▄█

            ╓▓▀         ▀▌     ▀█▄        ▄█

          ▄█▀             ▀▄     ╙▀▓▄    ▄▀

        ╓█▀                 ▀█╖     ╙▀▓▄█▀

       ██                     ╙▀▄╓    ▄▌

      █╛▀█                       ╙▀▓▄█▀

     ▓▌   █▄                       ▄█

     █░    ▀█┐                   ▄█╘

     █▌      ▀█╖               ▄█╙

      █─       ▀█▄           ▄█╙

      ▄█▄         ▀█▄     ┌▄▀└

     █▌ ▀█▄          ▀▀▄▄█▀

     ╙█▄▄▄▀▀▀▓▄▄▄▄▄▄▄▓▀▀

     
");
    let mut dirs_to_try = Vec::new();
    let mut subs_to_try = Vec::new();
    let args = Args::parse();
    let mut targets = Vec::new();
    for target in args.target.split(",").collect::<Vec<&str>>(){
        targets.push(target.to_owned());
    }
    if args.dirwordlist != String::from("none"){
        let dirwordlist = fs::read_to_string(args.dirwordlist).expect("error reading directory wordlist");
        for dir in dirwordlist.split("\n").collect::<Vec<&str>>(){
            dirs_to_try.push(dir.trim_end().trim_start().to_owned());
        }
    }
    if args.subwordlist != String::from("none"){
        let subwordlist = fs::read_to_string(args.subwordlist).expect("error reading subdomain word list");
        for sub in subwordlist.split("\n").collect::<Vec<&str>>(){
            subs_to_try.push(sub.trim_end().trim_start().to_owned())
        }
    }
    let mut domains_to_try = Vec::new();
    let mut urls_to_try = Vec::new();
    println!("loading targets, and wordlists...");
    for target in targets{
        if target.contains("/"){
            for dir in &dirs_to_try{
                let url = format!("{}/{}", target, dir);
                urls_to_try.push(url.to_owned());
            }
        }
        else if target.contains(".") == true{
            for sub in &subs_to_try{
                let domain = format!("{}.{}", sub, target);
                domains_to_try.push(domain.to_owned());
            }
        }
        else{
            println!("{} is not a valid target, please supply either a domain or url", target);
        }
    }
    let mut threads = Vec::new();
    let urls_len = urls_to_try.len();
    let domains_len = domains_to_try.len();
    println!("DONE!, {} URLS to try, and {} subdomains to try", &urls_len, &domains_len);
    if urls_len > 0{
        let dir_chunk_size = urls_len / &args.threads;
        let url_vecs: Vec<Vec<String>> = urls_to_try.chunks(dir_chunk_size).map(|x| x.to_vec()).collect();
        for urlvec in url_vecs{
            for url in urlvec{
                threads.push(thread::spawn(move || {
                    try_dir(url)
                }));
            }
        }
    }
    if domains_len > 0 {
        let domain_chunk_size = domains_len /&args.threads;
        let domain_vecs: Vec<Vec<String>> = domains_to_try.chunks(domain_chunk_size).map(|x| x.to_vec()).collect();
        for domainvec in domain_vecs{
            for domain in domainvec{
                threads.push(thread::spawn(move || {
                    try_sub(domain);
                }))
            }
        }
        for thread in threads{
            let _ = thread.join();
        }
    }
    println!("done bruteforcing, happy hunting!");
}
