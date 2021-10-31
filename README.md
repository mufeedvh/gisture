<h1 align="center"><code>gisture</code></h1>
<h4 align="center">Utilizing GitHub Gists as a Blogging Platform</h4>
<p align="center">
	<img src="https://img.shields.io/badge/version-0.1.0-blue.svg" title="version" alt="version">
</p>

A minimal and flexible blog generator based on GitHub Gists with SEO, Templating, Syntax Highlighting, and Metadata support out-of-the-box.

## Table of Contents

- [Features](#features)
- [Why Gist?](#why-gist)
- [Installation](#installation)
- [Usage](#usage)
- [Templating](#templating)
- [Modification Guide](#modification-guide)
- [Contribution](#contribution)
- [License](#license)
- [FAQ](#faq)

## Features

- **Single Binary** - Just run the binary and it will generate the starter boilerplate.
- **Simple Configuration** - A simple JSON file will be created upon initiation which has everything you need to setup your blog.
- **Templating** - A set of template variables are prefixed to write your own blog template or port any blog theme easily using everyone's familiar Handlebars. (See [Templating](#templating))
- **SEO Utility** - A `sitemap.xml` and `robots.txt` are automatically generated according to your gist entries.
- **Syntax Highlighting** - Every code snippet in your Gist will be highlighted in the generated HTML and you can add your own syntax spec with [Sublime Text syntax definitions](http://www.sublimetext.com/docs/3/syntax.html#include-syntax). (Thanks to [syntect](https://github.com/trishume/syntect))
- **Helpful Log Messages** - Every error case has been handled with a helpful and verbose error message to provide a breeze CLI experience.
- **Caching** - Since Gists are fetched from the API, building multiple blog entries will take time hence gisture handles a disk cache and only build when a gist is updated.

## Why Gist?

Here are some of the concepts that made me utilize/choose GitHub Gists:

- **Git:** You can use git to edit and manage a gist just like a repository.
- **Hosting:** Your Gists will exist as long as GitHub exists.
- **Integrated Comment Section:** Every gist has a comment section which supports Markdown.
- **Transparent:** As it's based on gists, anyone can check revisions to see the changes made to a blog entry.
- **Starring:** You can bookmark a gist entry into your GitHub account by starring it.

Also GitHub's Markdown editor is pretty cool and gisture uses [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) which supports GitHub flavored elements.

## Installation

    $ cargo install --git https://github.com/mufeedvh/gisture.git
    
[Install Rust/Cargo](https://rust-lang.org/tools/install)

## Build From Source

**Prerequisites:**

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```
$ git clone https://github.com/mufeedvh/gisture.git
$ cd gisture/
$ cargo build --release
```

The first command clones this repository into your local machine and the last two commands enters the directory and builds the source in release mode.

## Usage

A gisture blog should have `xyz.blog.md` as it's Gist filename where `/xyz` becomes the permalink, description as it's `meta description`, a Markdown title (`# Title`) for it's `title`.

Setup configuration and Generate template boilerplate:

    $ gisture

Build blog files with the configuration:

    $ gisture build

Open up a preview web server on port `1337`:

    $ gisture serve 1337

Just running `serve` will open up the web server on a random free port.

## Templating

gisture uses Handlebars as it's templating engine. All you need to make/port a theme for your blog, are these files and a couple of template variables which are automatically generated upon initiation.

### Screenshots

Here are some screenshots of the basic starter template. Admittedly it's not shiny and it's just made for demonstration purposes, the starter templates will show you how to use the template variables or port another theme to support gisture.

<table>
  <tr>
    <td><img src="https://user-images.githubusercontent.com/26198477/139580092-759947a6-d3f1-4f05-843b-fa7a7a6fa190.png"></td>
    <td><img src="https://user-images.githubusercontent.com/26198477/139580109-128ab3b2-18dd-4106-92b5-b5d4ec5d9631.png"></td>
    <td><img src="https://user-images.githubusercontent.com/26198477/139580104-37b8e494-b32e-4112-bfda-1d78cc978c0d.png"></td>
   </tr>
</table>

### Template Files

- `index.html` - The homepage.
- `page.html` - A blog/page entry.
- `page_list.html` - The blog listing element.
- `404.html` - Page Not Found template.

### Template Variables

**NOTE:** Just refer to the `templates/` directory to get up and running quickly, it has a starter template that utilizes these variables.

**Blog:**

- `{{ blog_title }}` - The home title of the blog.
- `{{ blog_description }}` - The home description of the blog.
- `{{ blog_url }}` - The URL of the blog.
- `{{ blog_list }}` - The list of all the blog/page entries as an HTML element. (`blog_list.html`)

**Gist:**

- `{{ page_title }}` - A blog/page entry's title.
- `{{ page_description }}` - A blog/page entry's description.
- `{{ page_url }}` - The full URL of a blog/page entry.
- `{{ published_date }}` - The published datetime of a blog/page entry.
- `{{ updated_at }}` - The recent update datetime of a blog/page entry.
- `{{ blog_contents }}` - The content of the blog/page entry.

## Modification Guide

Here are some code pointers if you want to modify gisture to fit your own needs or to add new features. I have tried to make the code verbose and easier to modify. :)

- [Markdown Parser Options](https://github.com/mufeedvh/gisture/blob/master/src/parsers.rs#L115-L123)
- [HTML Rewriting/Handling](https://github.com/mufeedvh/gisture/blob/master/src/parsers.rs#L49-L90)
- [Add New Configuration Options](https://github.com/mufeedvh/gisture/blob/master/src/config.rs#L11-L36)
- [Gist User Data](https://github.com/mufeedvh/gisture/blob/master/src/gist.rs#L14-L22)
- [Gist Entry Data](https://github.com/mufeedvh/gisture/blob/master/src/gist.rs#L149-L156)
- [Disk Caching](https://github.com/mufeedvh/gisture/blob/master/src/cache.rs#L6-L35)
- [SEO Function Utils](https://github.com/mufeedvh/gisture/blob/master/src/metadata.rs#L21-L24)
- [Syntax Highlighting Theme](https://github.com/mufeedvh/gisture/blob/master/src/parsers.rs#L45)

### API Guide

- [User's Gists](https://docs.github.com/en/rest/reference/gists#list-gists-for-a-user)
- [Single Gist](https://docs.github.com/en/rest/reference/gists#get-a-gist)
- [Gist Revision](https://docs.github.com/en/rest/reference/gists#get-a-gist-revision)
- [Gist Comments](https://docs.github.com/en/rest/reference/gists#list-gist-comments)

## Contribution

Ways to contribute:

- Suggest a feature
- Report a bug
- Fix something and open a pull request
- Help me document the code
- Spread the word
- Create a better starter template for gisture because I suck at CSS

## License
Licensed under the MIT License, see <a href="https://github.com/mufeedvh/gisture/blob/master/LICENSE">LICENSE</a> for more information.

## FAQ

1. Why?

> This is the embodiment of the [automation XKCD comic](https://xkcd.com/1319/), all I wanted to do was write a blog (which I didn't) and this is the result. I am not a fan of static site generators because of the markdown metadata section + the disqus comment hosting shenanigans (I don't like disqus) although [utteranc.es](https://utteranc.es/) is pretty cool. So I set out to find another static solution and ended up deciding to utilize Gist as a blogging platform because it comes with my favorite Markdown editor, an excellent comment section, starring for bookmarks and hence the yoink. Hopefully someone other than me who actually wants to write a blog finds it useful so putting it out there.

2. Why are the Handlebars variables unescaped?

> It's Markdown converted to HTML so it needs to be unescaped and one does not simply XSS their own blog.

## Liked the project?

Support the author by buying him a coffee!

<a href="https://www.buymeacoffee.com/mufeedvh" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="45" width="170"></a>

---
