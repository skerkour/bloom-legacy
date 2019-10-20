# Contributing to Bloom

We believes in a world where everyone can contribute. Our mission is to change
all creative work from read-only to read-write. When everyone can contribute,
users become contributors and we greatly increase the quality of global human life.


We want to make it as easy as possible for Bloom users to become Bloom contributors,
so we’ve created this guide to help you get started. We have multiple tracks to cater
to people of varying experience levels.


If you’re uncomfortable getting into open source development right away, you may
want to consider the Documentation or Translation tracks. Documentation and
Translation are both just as important as code, and we'd be happy to have your
contributions.

<!--
1. [Overview](#overview)
2. [Development](#development)
3. [UX / Design](#ux-design)
4. [Documentation](#documentation)
5. [Sponsoring](#sponsoring)
6. [Getting Help](#getting-help)

------------------------------- -->

# Overview

In order to coordinate our efforts we have 2 communication channels:

* <a href="https://gitlab.com/groups/bloom42/-/issues" target="_blank" rel="noopener">GitLab issues</a></li>
* <a href="https://discord.gg/HmDQDGv" target="_blank" rel="noopener">a Discord chat</a></li>


**GitLab issues** are are used to discuss about bugs and new features. They are our primary
project management tool (think Trello, but better). You can
<a href="https://www.buggycoder.com/project-management-with-gitlab" target="_blank" rel="noopener">read here</a> about how we use GitLab as our
project management tool.


**The Discord chat** is used for instant and informal communication. It must be used in resort,
and contributors should not rely on the long term archiving of the chat. GitLab issues are prefered to discuss new features.


# Development

These instructions are for development of any of Bloom project.

**Except for typos, Merge requests won't be accepted without opening an issue first.**
Merge requests must target the *dev* branch, therefore **merge requests targeting *master* won't be accepted**.

* Choose an issue to work on. Be sure to comment and verify no one else is working on the issue, and to make sure we’re still interested in a given contribution.<br/>
  You may also want to comment and ask for help if you’re new or if you get stuck. We’re more than happy to help!
* Add the feature or fix the bug you’ve chosen to work on.
* If it's a feature change that impacts users or admins, update the documentation.
* Open a merge request to merge your code and its documentation. The earlier you open a merge request, the sooner you can get feedback. You can mark it as a Work in Progress to signal that you’re not done yet.
* Add tests if needed
* Make sure the test suite is passing.
* Wait for a reviewer. You’ll likely need to change some things once the reviewer has completed a code review for your merge request. You may also need multiple reviews depending on the size of the change.
* Get your changes merged!



## Security notice

For security reasons please do not include any external dependency in a Merge request.

Please contact a member of the <a href="https://bloom.sh/about" target="_blank" rel="noopener">core team</a> if
  for some reason you think you need to include an external dependency.


### Reporting security issue

See our <a href="https://bloom.sh/security" target="_blank" rel="noopener">dedicated security page</a>.


# UX / Design

These indications are for those wanting to contribute UX designs specifically.
The UX department at Bloom uses Sketch for all of its designs. See the <a href="https://gitlab.com/bloom42/design">
Design repository</a> for details on working with our files.
Visit our Contributing guidelines to read our general guidelines for contributing.
While they are code-focused instructions, they will help you understand the overall process of contributing.


You can refer to the Development worflow to propose new features / fix bugs.


# Documentation

This section pertains to documentation changes that are independent of other
code/feature changes. For documentation changes that accompany changes to
code/features, see the Development section above.


Bloom's documentation is splitted in 2 parts:
* Documentation for users: <a href="https://bloom.sh/help" target="_blank" rel="noopener">Help</a>,
  you can find the repo here: <a href="https://gitlab.com/bloom42/server">https://gitlab.com/bloom42/server</a>
  You can open an issue with a proposal or open an merge request.
* *Documentation for developers: <a href="https://bloom.sh/open-source">https://bloom.sh/open-source</a>.
  Each project documentation can be found within the project's repository in the <b>docs</b> folder.


# Sponsoring

 Another way to contribute to the Bloom project is financially: If you want to have your logo
on our <a href="https://bloom.sh/about" target="_blank" rel="noopener">about page</a> or donate to make the world a better place, please see our
<a href="https://bloom.sh/become-a-sponsor" target="_blank" rel="noopener">Become a sponsor page</a>.


# Getting Help

If you need help while contributing to Bloom, below are some of the resources that are available:

* <a href="https://bloom.sh/help" target="_blank" rel="noopener">Our Help and Documentation website</a>
* <a href="https://discordapp.com/invite/HmDQDGv">The Discord chat</a>
* By email: <a href="mailto:hello@bloom.sh">hello@bloom.sh</a>

<!--
see also
See https://github.com/golang/go/blob/master/CONTRIBUTING.md
https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md
https://github.com/flutter/flutter/blob/master/CONTRIBUTING.md
https://about.gitlab.com/community/contribute/
https://about.gitlab.com/company/culture/contribute/
https://docs.gitlab.com/ee/development/contributing/
https://github.com/kubernetes/community/tree/master/contributors/guide
 -->
