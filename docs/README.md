# MicroSDeck Documentation

You have found the source for the MicroSDeck documetnation. Its all within the *.mdx files in this folder.

## Mdx?

[Mdx](https://mdxjs.com) is a superset of the [Md](https://www.markdownguide.org/) language. Both are human readable markup languages that allow styling of content without much hassle.

If you have used Markdown them MDX is essentially the same just with some JSX support which allows embedding of custom components into markup files.

If you haven't used markdown, then there is a chance that actually you have. Discord, Reddit, WhatsApp, Discourse & Most Fediverse apps all have markdown support. So if you know how to make text bold in discord, there is a good chance that you know how to make text bold in markdown.

If you haven't ever used any of those apps or never wrote anything but the most basic of unformatted text. Or even if you just want a reference or quick refresher. You can use the following Cheat Sheet to get up to date quickly:
[https://www.markdownguide.org/cheat-sheet/](https://www.markdownguide.org/cheat-sheet/)

## JSX?

If you are familiar with Markdown already you might be interested in learning a bit more about these custom components. Through the magic of [Mdx](https://mdxjs.com) we can import React components into the markdown page. These then get rendered together with the page into JSX components that are bundled into the final bundle. 

Since it all ends up as built JSX we can utilize the same exact logic as the rest of the MicroSDeck codebase to retrieve the MicroSDeck instance and any data it contains. Which all get live updated when the state changes. You can check out an [example here](./components/CurrentCard.tsx). 

This allows us to functionally do exactly the same as in the main ui. We can change the name of cards or update any other data we want. Which in the right hands can create a VERY interactive documentation page that both reacts to the state of the plugin as well as allows the users to change it from within the docs (Think a "Try Me" button).

## Contributing

Any and all contributions are welcome. Especially small fixes to correct spelling mistakes, grammer or sentence structure. Nothing is too small and any contributions are welcome. If you do make changes to the docs I ask only that the pullRequest be called `docs: [describe the change]` to keep consistency. 


## Thank You

Thank you so much for even reading this far. Now go off & have fun, My rambles surely weren't THAT interesting.