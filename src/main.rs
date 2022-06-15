use dialoguer::{theme::ColorfulTheme, Select};
use rand::{self, Rng};
extern crate urlshortener;
use urlshortener::{client::UrlShortener, providers::Provider};

mod script;

fn main() {
    let urls = [
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://www.youtube.com/watch?v=oHg5SJYRHA0",
        "https://www.youtube.com/watch?v=6_b7RDuLwcI",
        "https://www.youtube.com/watch?v=G8iEMVr7GFg",
        "https://www.youtube.com/watch?v=AyOqGRjVtls",
        "https://www.youtube.com/watch?v=6mhmcwmgWbA",
        "https://www.youtube.com/watch?v=SpZ2FsEfwP4",
        "https://www.youtube.com/watch?v=H01BwSD9eyQ",
        "https://www.youtube.com/watch?v=nrsnN23tmUA",
        "https://www.youtube.com/watch?v=8mkofgRW1II",
        "https://www.youtube.com/watch?v=rAx5LIul1N8",
        "https://www.youtube.com/watch?v=sO4wVSA9UPs",
        "https://www.youtube.com/watch?v=rrs0B_LM898",
        "https://www.youtube.com/watch?v=doEqUhFiQS4",
        "https://www.youtube.com/watch?v=epyRUp0BhrA",
        "https://www.youtube.com/watch?v=uK5WDo_3s7s",
        "https://www.youtube.com/watch?v=wzSVOcgKq04",
        "https://www.youtube.com/watch?v=7B--1KArxow",
        "https://www.youtube.com/watch?v=rbsPu1z3ugQ",
        "https://www.youtube.com/watch?v=ptw2FLKXDQE",
        "https://www.youtube.com/watch?v=E50L-JYWm3w",
        "https://www.youtube.com/watch?v=8leAAwMIigI",
        "https://www.youtube.com/watch?v=ByqFY-Boq5Y",
        "https://www.youtube.com/watch?v=E4ihJMQUmUQ",
        "https://www.youtube.com/watch?v=cjBHXvBYw5s",
        "https://www.youtube.com/watch?v=xaazUgEKuVA",
        "https://www.youtube.com/watch?v=TzXXHVhGXTQ",
        "https://www.youtube.com/watch?v=Uj1ykZWtPYI",
        "https://www.youtube.com/watch?v=EE-xtCF3T94",
        "https://www.youtube.com/watch?v=V-_O7nl0Ii0",
        "https://www.youtube.com/watch?v=cqF6M25kqq4",
        "https://www.youtube.com/watch?v=0SoNH07Slj0",
        "https://www.youtube.com/watch?v=xfr64zoBTAQ",
        "https://www.youtube.com/watch?v=j5a0jTc9S10",
        "https://www.youtube.com/watch?v=dPmZqsQNzGA",
        "https://www.youtube.com/watch?v=nHRbZW097Uk",
        "https://www.youtube.com/watch?v=BjDebmqFRuc",
        "https://www.youtube.com/watch?v=Gc2u6AFImn8",
        "https://www.youtube.com/watch?v=8VFzHYtOARw",
        "https://www.youtube.com/watch?v=cSAp9sBzPbc",
        "https://www.youtube.com/watch?v=Dx5i1t0mN78",
        "https://www.youtube.com/watch?v=Oo0twK2ZbLU",
        "https://www.youtube.com/watch?v=cvh0nX08nRw",
        "https://www.youtube.com/watch?v=lXMskKTw3Bc",
        "https://www.youtube.com/watch?v=7z_1E8VGJOw",
        "https://www.youtube.com/watch?v=VgojnNgmgVs",
        "https://www.youtube.com/watch?v=5wOXc03RwVA",
        "alturl.com/p749b",
        "asongbyagayguy.ytmnd.com",
        "bit.ly/3tjnLXy",
        "bit.ly/free-nitro-forever",
        "cdn.discordapp.com/attachments/875477771114147900/949605447755763743/SPOILER_unknown.png",
        "cutt.ly/yAxl79v",
        "dev--meiraba-utils.meiraba.autocode.gg",
        "discord.yonle.repl.co",
        "ebaumsworld.com/video/watch/411687",
        "fuckdreamhost.com",
        "is.gd/hello109",
        "kavin.rocks",
        "keiraknightley.ytmnd.com",
        "phantom.api.stdlib.com/rickroll-website@dev/",
        "pottermisleading.ytmnd.com",
        "r.mtdv.me",
        "rasher.dk/r",
        "rickrolled.fr",
        "rubyurl.com/1H1G#body",
        "rubyurl.com/1H1G$script",
        "secretrickroll.com",
        "shorturl.at/gtzVY",
        "smouch.net/lol",
        "stucknut.com/locker/files/jessica.gif",
        "t.ly/xzLR",
        "thisworldthesedays.com",
        "tinyurl.com/2g9mqh",
        "tinyurl.com/37ws8e",
        "tinyurl.com/ynupj4",
        "tinyurl.com/yowxeq",
        "tobi-x.com/kate_moss_nude",
        "tomorrowtides.com",
        "youtu.be/dQw4w9WgXcQ",
        "youtu.be/hVzINKRbekY",
        "youtube.com/get_video?video_id=-HRRGn6tNd4",
        "youtube.com/get_video?video_id=-umtVthRGJI",
        "youtube.com/get_video?video_id=1V_aE_Xdde8",
        "youtube.com/get_video?video_id=3HrSN7176XI",
        "youtube.com/get_video?video_id=3KANI2dpXLw",
        "youtube.com/get_video?video_id=4TJB9FQo45E",
        "youtube.com/get_video?video_id=4ynGOr9vmyc",
        "youtube.com/get_video?video_id=5F5nc1bAaKw",
        "youtube.com/get_video?video_id=5kQKh2tdisQ",
        "youtube.com/get_video?video_id=5uZr3JWYdy8",
        "youtube.com/get_video?video_id=65I0HNvTDH4",
        "youtube.com/get_video?video_id=67KOFEEOhkI",
        "youtube.com/get_video?video_id=6SYVdI7Llrg",
        "youtube.com/get_video?video_id=8Set3JpJJ4w",
        "youtube.com/get_video?video_id=8aJjMOy-Ops",
        "youtube.com/get_video?video_id=A1sgzEDUG-o",
        "youtube.com/get_video?video_id=ABUhOJxZQmg",
        "youtube.com/get_video?video_id=AP12uZvfvag",
        "youtube.com/get_video?video_id=AS35zlAdaSQ",
        "youtube.com/get_video?video_id=CZoJt0Sbqrs",
        "youtube.com/get_video?video_id=DpPhnECPe2I",
        "youtube.com/get_video?video_id=EEbzptEFsKk",
        "youtube.com/get_video?video_id=FGEUClII8x0",
        "youtube.com/get_video?video_id=G_vas-7a7is",
        "youtube.com/get_video?video_id=HiaBjpzLgQI",
        "youtube.com/get_video?video_id=I6_0tpqg3ZE",
        "youtube.com/get_video?video_id=IHKAgwIxUAY",
        "youtube.com/get_video?video_id=IVvl_R2jYMo",
        "youtube.com/get_video?video_id=IpjGmx2v6bM",
        "youtube.com/get_video?video_id=J3VnZMoh7sk",
        "youtube.com/get_video?video_id=Kmt4wrn1MTk",
        "youtube.com/get_video?video_id=LeSnAn-Sc0g",
        "youtube.com/get_video?video_id=LqXTU8YAGzo",
        "youtube.com/get_video?video_id=MJCH_wT9TaU",
        "youtube.com/get_video?video_id=MoUPdJrjkCM",
        "youtube.com/get_video?video_id=NZ-AAD7Ci_c",
        "youtube.com/get_video?video_id=PIMrL4qXtJ0",
        "youtube.com/get_video?video_id=PXrHCBoEj7g",
        "youtube.com/get_video?video_id=QumbExFAj-U",
        "youtube.com/get_video?video_id=R5P1_U7LZX8",
        "youtube.com/get_video?video_id=RSsJ19sy3JI",
        "youtube.com/get_video?video_id=RgIDuaxiT0w",
        "youtube.com/get_video?video_id=RzoZGNsJ71w",
        "youtube.com/get_video?video_id=SGi7qi_y0Jw",
        "youtube.com/get_video?video_id=UDTGvgE5eJw",
        "youtube.com/get_video?video_id=VBY4TV5qK-4",
        "youtube.com/get_video?video_id=VVjUWKSZSsc",
        "youtube.com/get_video?video_id=WpEVccrkYQ0",
        "youtube.com/get_video?video_id=X-j828DqqnY",
        "youtube.com/get_video?video_id=XfTUDW93z6E",
        "youtube.com/get_video?video_id=Xsvi9uNrDSI",
        "youtube.com/get_video?video_id=YWn54TjfBkk",
        "youtube.com/get_video?video_id=Yu_moia-oVI",
        "youtube.com/get_video?video_id=ZIQZHqNQODo",
        "youtube.com/get_video?video_id=ZOU8GIRUd_g",
        "youtube.com/get_video?video_id=Zc2tpMgz6MI",
        "youtube.com/get_video?video_id=_0719DxMOUY",
        "youtube.com/get_video?video_id=_FyXC-SsAPE",
        "youtube.com/get_video?video_id=aSzhxllE0RM",
        "youtube.com/get_video?video_id=atiNprQmjks",
        "youtube.com/get_video?video_id=b1WWpKEPdT4",
        "youtube.com/get_video?video_id=b43GgNbz9fg",
        "youtube.com/get_video?video_id=cjeogv9VUAE",
        "youtube.com/get_video?video_id=cxwxBheZniM",
        "youtube.com/get_video?video_id=dS9DO6kx-Ek",
        "youtube.com/get_video?video_id=eKDcl0V6o6k",
        "youtube.com/get_video?video_id=eLiXPfl8EPY",
        "youtube.com/get_video?video_id=f2b1D5w82yU",
        "youtube.com/get_video?video_id=f8FHqEIzSlE",
        "youtube.com/get_video?video_id=fMzkMpMraFY",
        "youtube.com/get_video?video_id=fmxGLQd3J0U",
        "youtube.com/get_video?video_id=gLVtavM962w",
        "youtube.com/get_video?video_id=gdpijMRhoT8",
        "youtube.com/get_video?video_id=gvUiLtwlEl8",
        "youtube.com/get_video?video_id=hq05IRf0dBQ",
        "youtube.com/get_video?video_id=hyX_krrTBZ0",
        "youtube.com/get_video?video_id=iJbwc4wm5Y0",
        "youtube.com/get_video?video_id=lAQIiNTH46I",
        "youtube.com/get_video?video_id=lfO4Z5WEUuE",
        "youtube.com/get_video?video_id=lfao5IToml4",
        "youtube.com/get_video?video_id=lp6nr-rD_g8",
        "youtube.com/get_video?video_id=lzSjyzqfegI",
        "youtube.com/get_video?video_id=oHg5SJYRHA0",
        "youtube.com/get_video?video_id=piLTpv1eKdU",
        "youtube.com/get_video?video_id=q6NwqFlctZY",
        "youtube.com/get_video?video_id=r8tXjJL3xcM",
        "youtube.com/get_video?video_id=rfp7FbsnsbU",
        "youtube.com/get_video?video_id=tazYxtJcwCc",
        "youtube.com/get_video?video_id=thNTPin2RBY",
        "youtube.com/get_video?video_id=uYMIMPVK1vU",
        "youtube.com/get_video?video_id=ub_VBGajh-s",
        "youtube.com/get_video?video_id=uwnuL5Fy5g8",
        "youtube.com/get_video?video_id=uxIsiTo4VJo",
        "youtube.com/get_video?video_id=v7cyQ-dVaAM",
        "youtube.com/get_video?video_id=veFrQTKQy7A",
        "youtube.com/get_video?video_id=vf79MCuQ8jM",
        "youtube.com/get_video?video_id=vp8fkB0uwd0",
        "youtube.com/get_video?video_id=vvs7cXmVwN8",
        "youtube.com/get_video?video_id=xAp3HqpE7V8",
        "youtube.com/get_video?video_id=xm_EMOdpDhc",
        "youtube.com/get_video?video_id=xpupxRzumYs",
        "youtube.com/get_video?video_id=y5Ja-E529sU",
        "youtube.com/get_video?video_id=yXGqsnkLg0A",
        "youtube.com/get_video?video_id=ywoqy9PBN-0",
        "youtube.com/get_video?video_id=yxnWl63Avo4",
        "youtube.com/get_video?video_id=z-HWXfRKkJU",
        "youtube.com/get_video?video_id=z2kThcO6ig8",
        "youtube.com/get_video?video_id=zGm0nGF_y2E",
        "youtube.com/shorts/BUykFA7FCo4",
        "youtube.com/watch?v=0SoNH07Slj0",
        "youtube.com/watch?v=34Ig3X59_qA",
        "youtube.com/watch?v=5wOXc03RwVA",
        "youtube.com/watch?v=6-HUgzYPm9g",
        "youtube.com/watch?v=6_b7RDuLwcI",
        "youtube.com/watch?v=6mhmcwmgWbA",
        "youtube.com/watch?v=7B--1KArxow",
        "youtube.com/watch?v=7z_1E8VGJOw",
        "youtube.com/watch?v=8VFzHYtOARw",
        "youtube.com/watch?v=8leAAwMIigI",
        "youtube.com/watch?v=8mkofgRW1II",
        "youtube.com/watch?v=AyOqGRjVtls",
        "youtube.com/watch?v=BjDebmqFRuc",
        "youtube.com/watch?v=ByqFY-Boq5Y",
        "youtube.com/watch?v=DLzxrzFCyOs",
        "youtube.com/watch?v=Dx5i1t0mN78",
        "youtube.com/watch?v=E4ihJMQUmUQ",
        "youtube.com/watch?v=EE-xtCF3T94",
        "youtube.com/watch?v=G8iEMVr7GFg",
        "youtube.com/watch?v=Gc2u6AFImn8",
        "youtube.com/watch?v=H01BwSD9eyQ",
        "youtube.com/watch?v=IdkCEioCp24",
        "youtube.com/watch?v=Oo0twK2ZbLU",
        "youtube.com/watch?v=Rtqkxkt7Hyg",
        "youtube.com/watch?v=SpZ2FsEfwP4",
        "youtube.com/watch?v=TOcAAaZ1HhU",
        "youtube.com/watch?v=TzXXHVhGXTQ",
        "youtube.com/watch?v=Uj1ykZWtPYI",
        "youtube.com/watch?v=V-_O7nl0Ii0",
        "youtube.com/watch?v=VbUuB1aN2DA",
        "youtube.com/watch?v=VgojnNgmgVs",
        "youtube.com/watch?v=ZzUsKizhb8o",
        "youtube.com/watch?v=a3Z7zEc7AXQ",
        "youtube.com/watch?v=cSAp9sBzPbc",
        "youtube.com/watch?v=cqF6M25kqq4",
        "youtube.com/watch?v=cvh0nX08nRw",
        "youtube.com/watch?v=dPmZqsQNzGA",
        "youtube.com/watch?v=dQw4w9WgXcQ",
        "youtube.com/watch?v=doEqUhFiQS4",
        "youtube.com/watch?v=epyRUp0BhrA",
        "youtube.com/watch?v=gvGyS5j9aFY",
        "youtube.com/watch?v=hLZW4U86cL4",
        "youtube.com/watch?v=iik25wqIuFo",
        "youtube.com/watch?v=j5a0jTc9S10",
        "youtube.com/watch?v=lXMskKTw3Bc",
        "youtube.com/watch?v=nHRbZW097Uk",
        "youtube.com/watch?v=nrsnN23tmUA",
        "youtube.com/watch?v=oHg5SJYRHA0",
        "youtube.com/watch?v=oT3mCybbhf0",
        "youtube.com/watch?v=ptw2FLKXDQE",
        "youtube.com/watch?v=r7VYFiPoZk4",
        "youtube.com/watch?v=r8tXjJL3xcM",
        "youtube.com/watch?v=rAx5LIul1N8",
        "youtube.com/watch?v=rbsPu1z3ugQ",
        "youtube.com/watch?v=rrs0B_LM898",
        "youtube.com/watch?v=sO4wVSA9UPs",
        "youtube.com/watch?v=tzYPr9w22VU",
        "youtube.com/watch?v=uK5WDo_3s7s",
        "youtube.com/watch?v=wpV-gGA4PSk",
        "youtube.com/watch?v=wuggc60TVzU",
        "youtube.com/watch?v=wzSVOcgKq04",
        "youtube.com/watch?v=xaazUgEKuVA",
        "youtube.com/watch?v=xfr64zoBTAQ",
        "youtube.com/watch?v=xvFZjo5PgG0",
        "youtube.com/watch?v=zL19uMsnpSU",
    ];
    let random_url = urls[rand::thread_rng().gen_range(0..urls.len())];

    let selections = &[
        "redirect-script(redirects target to a rickroll link)",
        "random-url(gives a random url)",
        "",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("what rickroll do you want to generate?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("you selected: {}!", selections[selection]);
    if selections[selection] == "redirect-script(redirects target to a rickroll link)" {
        script::generate_redirect_script(random_url.to_string());
    } else if selections[selection] == "random-url(gives a random url)" {
        generate_urls(random_url.to_string());
    } else {
        println!("you selected nothing");
    }
}

fn generate_urls(random_url: String) {
    println!("{}", random_url);

    let selections = &["is.gd", "None"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select url shortening service")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("you selected: {}!", selections[selection]);

    if selections[selection] == "is.gd" {
        shorten_url(random_url.to_string());
    } else if selections[selection] == "None" {
        println!("{}", random_url);
    } else {
        println!("you selected nothing");
    }
}

fn shorten_url(random_url: String) {
    let long_url = random_url;
    let us = UrlShortener::new().unwrap();
    let short_url = us.generate(long_url, &Provider::IsGd);

    println!("Your url is: {:?}", short_url.unwrap().as_str());
}
