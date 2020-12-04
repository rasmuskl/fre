# FREcency tracking (`fre`)

`fre` is a CLI tool for tracking your most-used directories and files. 
Though inspired by tools like `autojump` or the `z` plugin for `zsh`, it takes a slightly 
different approach to tracking and providing usage data. 
The primary difference is `fre` does not support jumping. Instead, 
it just keeps track of and provides sorting methods for directories, 
which can then be filtered by another application like `fzf`, 
which does a much better job of filtering than something I can write.
Additionally, it uses an algorithm in which the weights of each directory
decay exponentially, so more recently used directories are ranked more highly
in a smooth manner.


## Usage

`fre` is primarily designed to interface with `fzf`. For general usage, 
a user will create a shell hook that adds a directory every time the current 
directory is changed. This will start to build your profile of most-used directories. 
Then, `fre` can be used as a source for `fzf`. I personally use the `fzf`-provided 
control-T bindings, modified to use `fre` as input. Some examples are below.

Basic usage
```sh
# Print directories, sorted by frecency, then pipe to fzf
fre --sorted | fzf --no-sort

# Print directories and their associated frecency, sorted by frecency
fre --stat

# Log a visit to a directory
fre --add /home/user/new_dir

# Decrease weight of a directory by 10 visits
fre --decrease 10 /home/user/too_high_dir

# Print directories and the time since they were last visited in hours
fre --stat --sort_method recent

# Print directories and the number of times they've been visited
fre --stat --sort_method frequent
```

## Installation

To install the latest version from source, simply `cargo install fre` or `cargo install --path .` from a cloned copy of this repo.

Other installation methods:
Arch Linux: `yay -S fre`

### `fzf` integration
For integration with `fzf` CTRL-T, define the following environment variables 
```zsh
export FZF_CTRL_T_COMMAND='command fre --sorted'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

To preferentially use results from `fre`, but fall back to other results, we can use 
`cat` to combine results before sending them to `fzf`. My favorite alternate source 
is `fd` ([link](https://github.com/sharkdp/fd)), but the more common `find` can also be 
used. The following options first use `fre` results, then use all the subdirectories 
of the current directory, then use every subdirectory in your home directory. 
This is what I personally use.

```zsh
export FZF_CTRL_T_COMMAND='command cat <(fre --sorted) <(fd -t d) <(fd -t d . ~)'
export FZF_CTRL_T_OPTS='--tiebreak=index'
```

### Shell integration

#### zsh
(credit to `autojump`)

```zsh
fre_chpwd() {
  fre --add "$(pwd)"
}
typeset -gaU chpwd_functions
chpwd_functions+=fre_chpwd
```

More shells to come

### Vim integration

Want to track what files are most frecently opened in vim? Me too. I'm working on making that functional.


## Comparison to existing solutions

The three projects I'm familiar with that are closest in function to this are `autojump`, the `z` shell plugin, and the `d` portion (and maybe the `f` in the future) of `fasd`. 

The primary difference from the rest of these is its reliance on a tool like `fzf` to provide any solid directory jumping functionality. This was an intentional choice, sticking to the Unix philosophy of "do one thing, and do it well". 

The other major change from these pieces of software is the algorithm used to rank directories.  `autojump` uses the following formula:

```python

def add_path(data, path, weight=10):
    # ...
    data[path] = sqrt((data.get(path, 0) ** 2) + (weight ** 2))
    # ...
```

Looking at it closely, it seems to just be calculating the hypotenuse of a triangle where one side is the length of the previous weight and the other is the length of the weight being added. This does not take into account time passed since access at all, which is not ideal since I would rather not have directories from years ago ranked highly.

`fasd` and `z` both use the same frecency function that looks something like this:

```zsh
function frecent(rank, time) {
    dx = t-time
    if( dx < 3600 ) return rank*4
    if( dx < 86400 ) return rank*2
    if( dx < 604800 ) return rank/2
    return rank/4
}
```

This works fine until you re-visit an old directory. Then, suddenly, `dx` is small again and all the old visits are re-weighted to `rank*4`, causing it to jump up in the sorted output. This is not really ideal. I want to be able to re-visit an old directory once without messing up my directory ranking. 

`fre` uses a frecency algorithm where the weight of a directory visit decays over time. Given a list of visit times (bold x), the frecency of the directory would look something like this (using lambda as the half life and "now" as the current time at calculation):

<span class="katex-display"><span class="katex"><span class="katex-mathml"><math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><semantics><mrow><mi>w</mi><mo stretchy="false">(</mo><mtext mathvariant="bold">x</mtext><mo stretchy="false">)</mo><mo>=</mo><munderover><mo>∑</mo><mrow><mi>i</mi><mo>=</mo><mn>0</mn></mrow><mi>n</mi></munderover><msup><mrow><mo fence="true">(</mo><mfrac><mn>1</mn><mn>2</mn></mfrac><mo fence="true">)</mo></mrow><mrow><mrow><mo fence="true">(</mo><msub><mi>t</mi><mrow><mi>n</mi><mi>o</mi><mi>w</mi></mrow></msub><mo>−</mo><msub><mi>x</mi><mi>i</mi></msub><mo fence="true">)</mo></mrow><mi mathvariant="normal">/</mi><mi>λ</mi></mrow></msup></mrow><annotation encoding="application/x-tex">w(\textbf{x}) = \sum_{i=0}^n\left(\frac{1}{2}\right)^{\left(t_{now} - x_i\right)/\lambda}</annotation></semantics></math></span><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height: 1em; vertical-align: -0.25em;"></span><span style="margin-right: 0.02691em;" class="mord mathnormal">w</span><span class="mopen">(</span><span class="mord text"><span class="mord textbf">x</span></span><span class="mclose">)</span><span class="mspace" style="margin-right: 0.277778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right: 0.277778em;"></span></span><span class="base"><span class="strut" style="height: 3.00557em; vertical-align: -1.27767em;"></span><span class="mop op-limits"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height: 1.6514em;"><span class="" style="top: -1.87233em; margin-left: 0em;"><span class="pstrut" style="height: 3.05em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mathnormal mtight">i</span><span class="mrel mtight">=</span><span class="mord mtight">0</span></span></span></span><span class="" style="top: -3.05001em;"><span class="pstrut" style="height: 3.05em;"></span><span class=""><span class="mop op-symbol large-op">∑</span></span></span><span class="" style="top: -4.30001em; margin-left: 0em;"><span class="pstrut" style="height: 3.05em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mathnormal mtight">n</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height: 1.27767em;"><span class=""></span></span></span></span></span><span class="mspace" style="margin-right: 0.166667em;"></span><span class="minner"><span class="minner"><span class="mopen delimcenter" style="top: 0em;"><span class="delimsizing size3">(</span></span><span class="mord"><span class="mopen nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height: 1.32144em;"><span class="" style="top: -2.314em;"><span class="pstrut" style="height: 3em;"></span><span class="mord"><span class="mord">2</span></span></span><span class="" style="top: -3.23em;"><span class="pstrut" style="height: 3em;"></span><span class="frac-line" style="border-bottom-width: 0.04em;"></span></span><span class="" style="top: -3.677em;"><span class="pstrut" style="height: 3em;"></span><span class="mord"><span class="mord">1</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height: 0.686em;"><span class=""></span></span></span></span></span><span class="mclose nulldelimiter"></span></span><span class="mclose delimcenter" style="top: 0em;"><span class="delimsizing size3">)</span></span></span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span class="vlist" style="height: 1.7279em;"><span class="" style="top: -3.9029em; margin-right: 0.05em;"><span class="pstrut" style="height: 2.7em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="minner mtight"><span class="mopen mtight delimcenter" style="top: 0em;"><span class="mtight">(</span></span><span class="mord mtight"><span class="mord mathnormal mtight">t</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height: 0.164543em;"><span class="" style="top: -2.357em; margin-left: 0em; margin-right: 0.0714286em;"><span class="pstrut" style="height: 2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mathnormal mtight">n</span><span class="mord mathnormal mtight">o</span><span style="margin-right: 0.02691em;" class="mord mathnormal mtight">w</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height: 0.143em;"><span class=""></span></span></span></span></span></span><span class="mbin mtight">−</span><span class="mord mtight"><span class="mord mathnormal mtight">x</span><span class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height: 0.328086em;"><span class="" style="top: -2.357em; margin-left: 0em; margin-right: 0.0714286em;"><span class="pstrut" style="height: 2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mathnormal mtight">i</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height: 0.143em;"><span class=""></span></span></span></span></span></span><span class="mclose mtight delimcenter" style="top: 0em;"><span class="mtight">)</span></span></span><span class="mord mtight">/</span><span class="mord mathnormal mtight">λ</span></span></span></span></span></span></span></span></span></span></span></span></span>

With a little bit of mathemagics, we don't actually have to store the vector of access times. We can compress everything down into one number as long as we're okay not being able to dynamically change the half life. 

This algorithm provides a much more intuitive implementation of frecency that tends to come up with results that more closely match those we would naturally expect.

## Support

I use this regularly on MacOS and Linux. I wrote it to be usable on Windows as well, 
but I don't run any tests for it. Caveat emptor.


## Stability

I've been using this for over a year with no chnages now, and it does everything I need it to do. I'm happy to add features or accept changes if this is not the case for you.

## About the algorithm

The algorithm used combines the concepts of frequency and recency into a single, sortable statistic called "frecency".
To my knowledge, this term was first coined by Mozilla to describe their URL suggestions algorithm. 
In fact, Mozilla already came up with nearly this exact algorithm and 
[considered using it to replace Firefox's frecency algorithm](https://wiki.mozilla.org/User:Jesse/NewFrecency?title=User:Jesse/NewFrecency).
The algorithm is also very similar to the cache replacement problem, and a more formal treatment of the
math behind it can be found in this [IEEE article](https://ieeexplore.ieee.org/document/970573) (sorry for the paywall).

