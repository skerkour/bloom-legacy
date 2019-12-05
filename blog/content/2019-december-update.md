+++
date = 2019-12-05T10:00:00+01:00
title = "December update: Native applications are coming"
tags = ["rust", "flutter", "bloom"]

[extra]
lang = "en"
+++


It's been 6 months since <a href="/blog/bloom-a-free-and-open-source-google" target="_blank" rel="noopener">Bloom launched</a>.
6 incredible months with a lot of sweat and some sleepless nights.

But something even more incredible is coming during the next months: The launch of Bloom's native applications
featuring among other things end to end encryption and offline support.


## The past few months

First of all I want to apologize for the lack of public communication the past few months.

*Empowering people with open technologies* requires a lot of cross domain competencies, whether it be
finance, project management, marketing, software developement, cryptography and so on...

Being the sole person working on Bloom full time (Marina no longer have neough free time, she is
in an internship for her schooling) means that I need to aquire myself all this knowledge, and, more importantly,
how to apply it to create an awesome product. It take a lot of time, efforts, and focus. Time I cannot spend on developing Bloom.
Also I apologize for not merging all merge requests, but the project and requirements are moving really fast
and requires a full time job to keep the pace.

But the largest part is behind us and you can expect a lot more of development and openness the coming months.

## From cloud to local first

*Cloud apps like Google Docs and Trello are popular because they enable real-time collaboration with colleagues, and they make it easy for us to access our work from all of our devices. However, by centralizing data storage on servers, cloud apps also take away ownership and agency from users. If a service shuts down, the software stops functioning, and data created with that software is lost.* from [InkAndSwitch](https://www.inkandswitch.com/local-first.html).

But first of all, cloud is a business model. Cloud's first objective is not to empower users or ease their lives, but to
created recuring revenue streams for enterprises (and investors).

What would happen to users' data if the
enterprise go bakrupt? If the enteprise decides to kill the product because it's not enough profitable?
If the enterprise's cloud get hacked? If an embargo is promulgated? If internet is cut for some days/weeks?

Cloud create a dependency which is the exact opposite of enmpowerement.

### Native applications


The first step is to remove the cloud dependency. It means that Bloom should no longer be accessible through a web browser.
It means that Bloom should be usable even if you go far from internet for 3 months.



### Cryptography

For the better or the worse, computers made copying data virtually free. It's inevitable.
Wheter it be an ebook, an image a text file. Whether the data is public or private. In the long run,
any piece of data will become either public or it will be lost.

But the asymmetry between those who store and proceed the data (*cloud providers*), and those who produce
the data (*people*) create great power imbalances.

This is why, as of today, cryptography is the only mean to equilibrate power between those 2 categories.

With cryptography, **You** choose to whom you give access to **What**. **You** choose **Who** can
read your messages, not any random Facebook employee.

<div class="center">
  <img src="cia_triad.jpg" height="500"/>

  The C.I.A triad
</div>

Cryptography gives you confidentiality and integrity for your data, and paying for Bloom Drive gives you availability.


### Collaboration

Finally, humans operate in terms of networks (a.k.a communities, groups...) and this is why we have included
the *groups* feature, where you will be able to share a drive space, notes, tasks and so on.


## Summary

* The web application (<a href="https://legacy.bloom.sh" target="_blank" rel="noopener">https://legacy.bloom.sh</a>) is deprecated and all data will be deleted on February 29, 2020 at .
* Native applications are coming, featuring end to end encryption and offline support among other improvements
* **You will need to create a new account on the native applications**
* New Mastodon account: <a href="https://mastodon.social/@42bloom" target="_blank" rel="noopener">@42bloom@mastodon.social</a>
* Paying offers (Drive, Phaser, Bitflow...) will be available
* Next months will be dedicated to secure revenues and grow the core team

<br />
<br />
Have a great day ✌️
