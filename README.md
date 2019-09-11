# htmlq - a simple rough and ready command-line HTML processor

## Examples

```shell
$ http "https://tomafro.net" | htmlq "header a"
<a href="/">Tom Ward</a>
<a href="/projects">projects</a>
<a href="/links">links</a>
<a href="https://twitter.com/tomafro"><i class="fab fa-twitter-square"></i></a>
<a href="https://github.com/tomafro"><i class="fab fa-github-square"></i></a>
```

```shell
$ http "https://tomafro.net" | htmlq --output text "header a"
Tom Ward
projects
links


```

```shell
$ http "https://tomafro.net" | htmlq --output inner "header a"
Tom Ward
projects
links
<i class="fab fa-twitter-square"></i>
<i class="fab fa-github-square"></i>
```

## Motivation

* To get more experience writing rust
* To add some simple checks to my static site https://tomafro.net
