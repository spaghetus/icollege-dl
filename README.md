# iCollege Downloader

> *For when Mom spent a bunch of money on CompTIA study materials from a sketchy website without asking if you needed them and you want to listen to them like an audiobook so you can actually get some value out of them*

Using this software is simple.

1. Make sure Rust and `chromedriver` are installed.
2. `./driver.sh` - Start a remote-controllable instance of Chrome.
3. `EMAIL=... PASSWORD=... cargo run` - Use the instance of Chrome to generate a shell script with wget commands that will download all of your course videos.
   * This takes a while; please don't interfere with the Chrome window while it is underway. Doing so may cause downloads to be named incorrectly.
   * If it becomes stuck on the "edit profile" page, navigate to "my learning" manually. The process will continue.
4. `bash script.sh` - Run the shell script, which will download all of your videos to `./out`. This also takes a while, so don't interrupt it. If it is interrupted, you can recover by deleting the newest file in `./out` and starting over from step 3.

Don't distribute the files this software downloads or the scripts it generates. Just because it's trivial to scrape them doesn't mean they're in the public domain. As per the [LICENSE](LICENSE), I am not liable for anything you do with this software.