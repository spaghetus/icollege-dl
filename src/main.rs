#![feature(async_closure)]

use anyhow::Error;
use fantoccini::{ClientBuilder, Locator};
use std::io::Write;
use std::{any::Any, future::Future, io::Cursor, path::Path, process, time::Duration};
use tokio::time::Timeout;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let email = std::env::var("EMAIL")?;
	let password = std::env::var("PASSWORD")?;
	std::fs::create_dir("./out").unwrap_or(());
	#[cfg(feature = "script")]
	let mut script = Some(
		std::fs::OpenOptions::new()
			.create(true)
			.truncate(true)
			.write(true)
			.open("./script.sh")?,
	);
	let mut c = ClientBuilder::native()
		.connect("http://localhost:4444")
		.await
		.expect("failed to connect to WebDriver");
	// Go to the homepage
	c.goto("https://www.icollege.co/").await?;
	// Click "Login"
	c.find(Locator::Css("ul.nav>:nth-child(2)>span"))
		.await?
		.click()
		.await?;
	tokio::time::sleep(Duration::from_millis(1000)).await;
	c.wait_for_find(Locator::Css("div#LoginModal.show")).await?;
	// Fill in the e-mail
	c.find(Locator::Css("input#edit-lcgemail"))
		.await?
		.send_keys(email.as_str())
		.await?;
	// Fill in the password
	c.find(Locator::Css("input#edit-lcgpass"))
		.await?
		.send_keys(password.as_str())
		.await?;
	// Click "Login"
	c.find(Locator::Css("#edit-submit--19"))
		.await?
		.click()
		.await?;
	tokio::time::sleep(Duration::from_millis(1000)).await;
	c.wait_for_find(Locator::Css("a#scmdata")).await?;
	// Get a list of all courses
	let mut courses = vec![];
	for v in c
		.find_all(Locator::Css("div.course_listt div.row"))
		.await?
		.iter_mut()
	{
		let url = v
			.find(Locator::Css("a#scmdata"))
			.await?
			.attr("href")
			.await
			.unwrap()
			.unwrap();
		let name = v.find(Locator::Css("div.r_dtl>a")).await?.text().await?;
		courses.push((url, name));
	}
	// Visit each course
	for (url, name) in courses {
		c.goto(&url).await?;
		for (index, mut button) in c
			.find_all(Locator::Css("tr>td>a"))
			.await?
			.iter_mut()
			.map(|v| v.clone())
			.enumerate()
		{
			let name = format!(
				"{} {:02} {}",
				name,
				index,
				button.text().await?.clone().replace("WATCHED\n", "")
			);
			let path = format!(
				"./out/{}.mp4",
				name.replace(' ', "-").replace('\n', "-").replace('/', "-")
			);
			if !Path::new(&path).exists() {
				// Click several times to make sure it really does the thing
				button.click().await?;
				tokio::time::sleep(Duration::from_millis(3000)).await;
				c.wait_for_find(Locator::Css("video>source")).await?;
				c.execute(
					"document.querySelectorAll('video').forEach(v => {v.autoplay = false;v.pause()})",
					vec![],
				)
				.await?;
				let mut url = c.find(Locator::Css("video>source")).await?;
				let url = match url.attr("src").await? {
					Some(v) => v,
					None => continue,
				};
				#[cfg(feature = "script")]
				{
					// iCollege blocks the wget user agent, so we use a normal browser's user agent instead.
					writeln!(script.as_mut().unwrap(), "wget -U \"Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1)\" -O '{}' '{}'", path, url)?;
				}
				#[cfg(not(feature = "script"))]
				{
					let response = reqwest::get(url).await?;
					println!("Downloading {}", name);
					let mut file = std::fs::File::create(path)?;
					let mut content = Cursor::new(response.bytes().await?);
					std::io::copy(&mut content, &mut file)?;
					println!("Finished downloading {}", name);
				}
			} else {
				println!("Skipping existing video {}", name);
			}
		}
	}
	Ok(())
}
