use color_eyre::Report;
use yate::html;

fn template(inner: String) -> String {
    html! {

           <!DOCTYPE html>
           <html lang="en">

           <head>
               <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
               <link rel="icon" href="favicon.png"/>

               <meta charset="utf-8"/>
               <meta name="description" content="/usr/stories podcast"/>
               <meta content="width=device-width, initial-scale=1" name="viewport"/>
               <title>"/usr/stories podcast"</title>

               <style>"
                  body {
                    font: 100% Courier New, monospace;
                    margin: 20px;
                    //line-height: 26px;
                    background-color: #000000;
                    padding: 10px;
                    color: #84c754;
                  }
                  a {
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
                  }
            "</style>
           </head>

           {inner}

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
    <body class="glow">
      <center>
        <a href="">"Listen"</a>"&nbsp;&nbsp;"
        <a href="https://wandering.shop/@usrstories">"Follow us on Mastodon"</a>"&nbsp;&nbsp;"
        //<a href="credits.html">"Credits"</a>"&nbsp;&nbsp;"
        <a href="https://discord.gg/mCY2bBmDKZ">"Chat on Discord"</a>"&nbsp;&nbsp;"
        <a href="">"Support us on Patreon"</a>"&nbsp;&nbsp;"
        <br/>
        <br/>
        <h1><a href="">"/usr/stories"</a>" are hard scifi tales from the computer revolution."</h1>
        <div class="narrow">
        </div>
      </center>
      <br/>
      <br/>
      "
        $ cd /usr/stories<br>
        $ ls -al<br>
      "
      <div id="typed">r#"
        total 10<br/>
        -rw-r--r-- Jan  1 00:00 <a href="">episode0.mp3</a><br/>
        -rw-r--r-- Jan  1 00:00 episode1.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode2.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode3.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode4.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode5.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode6.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode7.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode8.mp3<br/>
        -rw-r--r-- Jan  1 00:00 episode9.mp3<br/>
        <a href="">$ play episode0.mp3</a><br/>
      "#</div>

    </body>
      })
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    std::fs::write("docs/index.html", index())?;
    println!("Built site OK!");
    Ok(())
}
