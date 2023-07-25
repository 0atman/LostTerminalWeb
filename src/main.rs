use color_eyre::Report;
use html_to_string_macro::html;

fn template(inner: String) -> String {
    let page_style = r#"
      body {
        font: 100% Courier New, monospace;
        margin: 20px;
        background-color: #000000;
        padding: 10px;
        color: #84c754;
        background-image: url('stars.jpeg');
        background-repeat: no-repeat;
        background-attachment: fixed;
        background-size: cover;
      }
      a, .white, h1, h2 {
        color: white;
      }
      .glow {
        color: #84c754;
        -webkit-animation: glow 1s linear infinite alternate;
        -moz-animation: glow 1s linear infinite alternate;
        animation: glow 1s linear infinite alternate;
        text-shadow: 0 0 5px #fff;
      }
      @-webkit-keyframes glow {
        from {
          text-shadow: 0 0 10px #fff;
        }
        to {
          text-shadow: 0 0 15px #fff;
        }
      }
      .narrow {
         max-width: 50%;
      }
      h1 {
        max-width: 40rem;
        line-height: 3rem;
      }"#;
    html! {
           <!DOCTYPE html>
           <html lang="en">

           <head>
               <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
               <link rel="icon" href="favicon.png"/>

               <meta charset="utf-8"/>
               <meta name="description" content="How do you learn to be human if there's no-one around to teach you?"/>

               <meta content="width=device-width, initial-scale=1" name="viewport"/>
               <title>"Lost Terminal Podcast"</title>

               <style>

               {page_style}

               </style>

           </head>

    <body class="glow">
      <center>
        <a href="https://www.spreaker.com/show/lost-terminal">"Listen"</a>"&nbsp; "
        <a href="https://fosstodon.org/@lostterminal">"Follow me on Mastodon"</a>"&nbsp; "
        <a href="credits.html">"Credits"</a>"&nbsp; "
        <a href="seasons.html">"Seasons"</a>"&nbsp; "
        <a href="https://www.teepublic.com/user/lost-terminal">"Store"</a>"&nbsp; "
        <a href="https://discord.gg/mCY2bBmDKZ">"Chat on Discord"</a>"&nbsp; "
        <a href="https://www.patreon.com/lostterminalpod">"Support me on Patreon"</a>"&nbsp; "
        <br/>
        <br/>


        <a href="index.html">
        <img src="logo.png" width="50%"/>
        </a>
    </center>
           {inner}


    </body>
     <script src="https://unpkg.com/typeit@8.7.1/dist/index.umd.js"></script>
     <script language="javascript">
     r##"
     new TypeIt("#typed", {
        speed: 1,
        loop: false,
        html: true,
        nextStringDelay: 0
      }).go();
      "##
     </script>

     </html>
    }
}

fn index() -> String {
    template(html! {
    <center>
              <div class="white"> "How do you learn to be human if there's no-one around to teach you?" </div>
              <br/>
              <br/>
          "A hopepunk podcast following the journey of a little satellite trying to understand what has happened after Earth stops returning his calls."
      <div class="narrow">
      </div>
    <br/>
    <br/>

    <div id="typed">"New episodes Mondays." "&nbsp;"

        <a href="https://www.spreaker.com/show/lost-terminal">"Listen here"</a>
    </div>
    </center>
    })
}

fn seasons() -> String {
    template(html! {


        <h1>"PLOT SUMMARIES FOR EACH SEASON"</h1>
        <h1>"BEWARE SPOILERS!"</h1>

        <h2>"1.0"</h2>
            "On Station 6, an ESA satellite, Seth the AI finds his generator is at long last winding down. Drastic measures must be made to find a new life for himself. With his trusted friend Antarctica and less-understood friend Peter back on earth, he enacts a wild escape plan."

        <h2>"2.0"</h2>
            "Having crashed the Shuttle, Seth wakes up in unfamiliar databanks in an unfamiliar location. He is on Earth! But who are his new friends, why did Antarctica betray him, and who lives in the cracked bunker at the top of the hill?"

        <h2>"3.0"</h2>
            "Now living in the bunker, Seth is able to pick up transmissions from farther afield, even all the way to the signal coming from just outside Geneva. It's a group of survivors, and they need help. "

        <h2>"4.0"</h2>
            "Seth and Ana meet a new friend, Minnie, whom they play a remote game of imagination and adventure with, while Seth wrestles with his dual nature."

        <h2>"5.0"</h2>
            "Antarctica needs resucing, and Seth and his new Svalbardian friends Pawel, Linda, Kamil, Yeshi, and Amelie, sail from the top of the world to the bottom. But will they arrive in time?"

        <h2>"6.0"</h2>
            "With Antarctica reluctantly rescued, the ship returns to the Novamediterra. Seth meets a fellow AI TASSI, who lives in California, and another, LUNA, who live on the far side of the moon."

        <h2>"7.0"</h2>
            "Seth's friend LUNA wants to have more contact with the world, and PETER has some trouble. As for Seth, he searches for his place in the world."

        <h2>"8.0"</h2>
            "A long-dormant ESA FAILSAFE satellite wakes and begins taking over the whole network. Maddie, with her new capable EQUUS legs, hitches a lift to the Netherlands, and seeks to stop it by finding ESA Mission Control."

        <h2>"9.0"</h2>
            "Seth and Maddie ride the Trans Siberian express, and make friends with the family that run the old train. All the while, IVAN and LUNA attempt to find god."

        <h2>"10.0"</h2>
            "Half way through their journey, the train breaks down in a large abandoned city. The family and Maddie must scavenge for food and parts to keep body and train together. But who is the voice in the city?"

        <h2>"11.0"</h2>
            "An injury causes Seth to regress into his memories, and he thinks he is back on Station 6, his mother and the crew are alive, and he has no memory of his time on Earth. The dream begins to crumble, but Seth wants to stay asleep."

        <h2>"12.0"</h2>
            "Repaired by the Omarov family, with remote guidance from Alexander, Seth returns to the Novamediterra on board the Molly Hughes II again. On the journey, they discover a great deal more life, both natural and mechanical, exists beneath the waves than they realised."
    })
}

fn credits() -> String {
    template(html! {
        <br/>
        <br/>

    <span class="is-family-monospace">
      "Every episode has headline credits within the episode, and extended credits in the descriptions, both on youtube and in the shownotes of the podcast."
      <br/>
      "However, Lost Terminal would not be what it is today without these people, who I would like to thank here too:"
      <br/>
      <br/>
      <br/>



      <h2 class="title has-text-white">"Voices"</h2>

      <div class="list">
          <li>"Credits voiced by Lucy Stringer"</li>
          <li>"Antarctica voiced by Wolfie Thorns"</li>
          <li>"Ivan voiced by Alex Bayly"</li>
          <li>"Ana voiced by Oriel Winslow"</li>
          <li>"MINI voiced by Kate Ashford"</li>
          <li>"Yeshi voiced by Robin Howell"</li>
          <li>"PETER voiced by Karl Williams"</li>
          <li>"Alexander voiced by Dan Yilmaz"</li>
          <li>"50Meg voiced by Carin Calder-La Croix"</li>
          <li>"Alec voiced by Neil Murton"</li>
          <li>"NANA voiced by Lisa Ashton-Riemers"</li>
          <li>"Dr Redwing voiced by Gina Sneesby"</li>
      </div>


      <br/>
      <br/>

      <h2 class="title has-text-white">"Producers"</h2>

      <div class="list">
          <li>"Ada Phillips"</li>
          <li>"Kit"</li>
          <li>"Wil Taylor"</li>
          <li>"Kit"</li>
          <li>"deeryeen"</li>
          <li>"Andrew Krieg"</li>
          <li>"Toby"</li>
          <li>"Jade Felicity Bilkey"</li>
      </div>


      <br/>
      <br/>

      <h2 class="title has-text-white">"Character Concepts"</h2>

          "Sometimes friends suggest ideas to me, and sometimes I include them! (after some modification). If you are a patron, look" <a href="https://www.patreon.com/posts/52133498">"here"</a> "for how your character ideas can be included in Lost Terminal."
          <br/>
          <br/>

      <div class="list">
          <li>"EMMA and Dr Redwing created by Robert Bettelheim"</li>
          <li>"IVAN created by Karl Williams"</li>
          <li>"Yeshi created by Petra Bačkovská"</li>
          <li>"TASSI created by CM-47"</li>
          <li>"NANA created by Sal Boye"</li>
      </div>


      <br/>
      <br/>

      <h2 class="title has-text-white">"Additional Material"</h2>
      <div class="list">
          <li>"Podcast artwork by Carl Huber" <a class="has-text-white" href="http://www.carlh.com">"(carlh.com)"</a></li>
      </div>
    </span>


        })
}

fn build(pages: Vec<(&str, fn() -> String)>) -> Result<(), Report> {
    for (page, fun) in pages {
        std::fs::write(page, fun())?;
    }
    Ok(())
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    let _ = build(vec![
        ("docs/index.html", index),
        ("docs/credits.html", credits),
        ("docs/seasons.html", seasons),
    ]);
    println!("Built site OK!");
    Ok(())
}
