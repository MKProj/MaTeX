# MaTeX 
MaTeX is a simpler version of LaTeX built for users who do not want to deal with 
the complications of LaTeX. This languages compiles down to `tex` before compiling down to 
`pdf`. 

Here's the rundown for MaTeX, there's four types of tokens: 
1. Element (`\foo{some value}` = `foo: some value`)
2. Literal (`\bar` = `bar;`)
3. Environment (`\begin{baz}...\end{baz}` = `baz > begin`)
4. Comment (`% lalalala` = `% lalalala`)

So let's see how these look in MaTeX: 

```matex
% \documentclass{article}
documentclass: article
% \author{Someone}
author: Someone 
% \title{Some Title}
title: Some Title
% \date{Date}
date: Date
% \usepackage{listings}
import: listings 

% \begin{document}
document > begin
    % \maketitle
    maketitle;
    % \input{anotherfile}
    input: anotherfile
% \end{document}
document > end   
```

> For inputs, make sure to put the name of the file not `name.matex` since it's already inferred. 
- To create a new MaTeX project: `matex new <project_name>`
- To build pdf from single file: `matex build <name>`
- To compile MaTeX project: `matex compile`