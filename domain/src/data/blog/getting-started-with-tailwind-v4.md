---
date: 2025-08-07 21:19:16
description: 'Learn how to get started with Tailwind CSS v4, exploring its new features, improved syntax, and enhanced developer experience for modern web development.'
draft: false
last_modified: 2025-08-10 19:50:07
tags: ['tailwind', 'web']
title: 'Getting Started with Tailwind v4'
---

## Foreword

[Tailwind CSS v4](https://tailwindcss.com/blog/tailwindcss-v4) has been released for quite some time now, and recently I had the opportunity to work with [mayone](https://github.com/mayone) on migrating our utility-based CSS framework. Through this migration process, we've gained valuable insights and practical experience that I'd like to share with the community.

The examples in this article use `tailwindcss 3.4.17` / `tailwindcss 4.1.11`.

## Removal of `tailwind.config.js`

Tailwind v4 adopts a CSS-first configuration approach, allowing us to move various configurations directly into `tailwind.css`.

### `content`

```js:tailwind.config.mjs(Tailwind_v3)
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './components/**/*.{ts,tsx}',
    './app/**/*.{ts,tsx}',
  ],
  // ...
}
```

In Tailwind v4, it adopts [automatic content detection](https://tailwindcss.com/blog/tailwindcss-v4#automatic-content-detection), which only automatically excludes content from `.gitignore` and all binary files. This can easily lead to unnecessary classes being generated. For example, when I use Leptos for development, the lifetime annotation `'static` can cause the `static` class to be bundled.

Therefore, I highly recommend using the approach suggested on the official website:

#### Single Directory [^1]

```css:tailwind.css(Tailwind_v4)
@import 'tailwindcss' source("../app/**/*.{ts,tsx}");
```

#### Multiple Directories [^2]

```css:tailwind.css(Tailwind_v4)
@import 'tailwindcss' source(none);

@source "../app/**/*.{ts,tsx}";
@source "../components/**/*.{ts,tsx}";
```

Additionally, you may need to configure blocklist-related settings to exclude unwanted classes. [^3] See the blocklist section below for more details.

### `theme`

In Tailwind v3, to override the default theme, you can use it like this:

```js:tailwind.config.mjs(Tailwind_v3)
const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
  // ...
  theme: {
    extend: {
      fontFamily: {
        'sans': ['Inter', ...defaultTheme.fontFamily.sans],
      },
      // ...
    },
  },
}
```

In Tailwind v4, you can just override the variable in `tailwind.css`:

```css:tailwind.css(Tailwind_v4)
@theme {
 --font-sans:
  Inter, ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji',
  'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';
}
```

You can also only override the default font family of sans:

```css:tailwind.css(Tailwind_v4)
@theme {
  --default-sans-font-family: Inter;
}
```

If you want to use [Radix Colors](https://www.radix-ui.com/colors) together with Tailwind:

```css:radix/indigo.css
/*
  https://www.radix-ui.com/docs/colors/palette-composition/the-scales#indigo
*/

:root,
:host {
  --indigo-1: oklch(.994 0.0013 286.38);
  /* ... */
  --indigo-12: oklch(.313 0.0858 268.6);

  --indigo-alpha-1: oklch(.271 0.1879 264.052 / 0.78%);
  /* ... */
  --indigo-alpha-12: oklch(.208 0.1035 262.86 / 87.84%);
}

:root.dark,
:host.dark {
  --indigo-1: oklch(.191 0.0246 276.53);
  /* ... */
  --indigo-12: oklch(.911 0.0427 269.55);

  --indigo-alpha-1: oklch(.487 0.2894 265.14 / 5.88%);
  /* ... */
  --indigo-alpha-12: oklch(.911 0.0427 269.55);
}
```

```js:tailwind.config.mjs(Tailwind_v3)
/** @type {import('tailwindcss').Config} */
module.exports = {
  // ...
  darkMode: ['class'],
  theme: {
    colors: {
      primary: {
        DEFAULT: 'hsla(var(--primary))',
        foreground: 'hsl(var(--primary-foreground))',
      },
      // ...
    },
  },
}
```

```css:tailwind.css(Tailwind_v3)
@import "radix/indigo.css";

@layer base {
  :root {
    --primary: var(--indigo-9);
    --primary-foreground: var(--color-white);
  }

  .dark {
    --primary: var(--indigo-11);
    --primary-foreground: var(--color-black);
  }
}
```

In Tailwind v4, we can use new directives [`@variant`](https://tailwindcss.com/docs/functions-and-directives#variant-directive) and [`@custom-variant`](https://tailwindcss.com/docs/adding-custom-styles#adding-custom-variants):

```css:radix/indigo.css(Tailwind_v4)
/*
  https://www.radix-ui.com/docs/colors/palette-composition/the-scales#indigo
*/

:root,
:host {
  --indigo-1: oklch(.994 0.0013 286.38);
  /* ... */
  --indigo-12: oklch(.313 0.0858 268.6);

  --indigo-alpha-1: oklch(.271 0.1879 264.052 / 0.78%);
  /* ... */
  --indigo-alpha-12: oklch(.208 0.1035 262.86 / 87.84%);

  @variant dark {
    --indigo-1: oklch(.191 0.0246 276.53);
    /* ... */
    --indigo-12: oklch(.911 0.0427 269.55);

    --indigo-alpha-1: oklch(.487 0.2894 265.14 / 5.88%);
    /* ... */
    --indigo-alpha-12: oklch(.911 0.0427 269.55);
  }
}
```

And then import it in this way [^4]:

```css:tailwind.css(Tailwind_v4)
@import "radix/indigo.css" layer(theme);

@custom-variant dark (&:where(.dark, .dark *));

@theme inline {
  --color-primary: var(--primary);
}

@layer theme {
  html,
  :host {
    --primary: var(--indigo-9);
    --primary-foreground: var(--color-white);

    @variants dark {
      --primary: var(--indigo-11);
      --primary-foreground: var(--color-black);
    }
  }
}
```

You might notice that `--color-primary` follows [Theme Variable Namespaces](https://tailwindcss.com/docs/theme#theme-variable-namespaces). By the way, there are some less common namespaces, for example [^5]:

```css:tailwind.css(Tailwind_v4)
@import 'radix/red.css' layer(theme);
@import 'radix/yellow.css' layer(theme);
@import 'radix/blue.css' layer(theme);
@import 'radix/purple.css' layer(theme);

@theme {
  --background-image-rainbow-gradient: linear-gradient(
    var(--red-9),
    var(--yellow-9),
    var(--blue-9),
    var(--purple-9)
  );
}
```

```tsx:rainbow-box.tsx
const RainbowBox = () => {
  return (
    <div className="bg-rainbow-gradient size-10" />
  );
};
```

### `blocklist` [^3]

```js:tailwind.config.mjs(Tailwind_v3)
/** @type {import('tailwindcss').Config} */
module.exports = {
  // ...
  blocklist: ['static'],
}
```

```css:tailwind.css(Tailwind_v4)
@source not inline("static");
```

### `safelist`

```js:tailwind.config.mjs(Tailwind_v3)
/** @type {import('tailwindcss').Config} */
module.exports = {
  // ...
  safelist: [
    {
      pattern: /^p[xybtlr]?-([1-9]|10)$/,
      variants: ['responsive', 'hover', 'focus']
    },
  ],
}
```

```css:tailwind.css(Tailwind_v4)
@source inline("p{x,y,t,b,l,r}-{1..10}");
```

### `plugins`

Tailwind CSS plugins are extensions that add new utility classes.

```js:tailwind.config.mjs(Tailwind_v3)
/** @type {import('tailwindcss').Config} */
module.exports = {
  // ...
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}
```

```css:tailwind.css(Tailwind_v4)
@plugin "@tailwindcss/forms";
@plugin '@tailwindcss/typography';
```

## Adding Custom Utilities [^6]

You can add custom utilities to Tailwind v4 by adding them to the `utilities` layer with `@utility` directive:

```css:tailwind.css(Tailwind_v4)
@utility text-0 {
  font-size: 0;
}
```

## New Utility Functions [^7]

Tailwind v4 introduces several convenient utility functions that enhance the developer experience and provide more flexibility in styling.

### `--alpha()`

```css:tailwind.css(Tailwind_v4)
.my-element {
  color: --alpha(var(--color-indigo-700) / 50%);
}
```

```css:output.css(compiled)
.my-element {
  color: color-mix(in oklab, var(--color-indigo-700) 50%, transparent);
}
```

### `--spacing()`

```css:tailwind.css(Tailwind_v4)
.my-element {
  padding: --spacing(2) --spacing(4);
}
```

```css:output.css(compiled)
.my-element {
  padding: calc(var(--spacing) * 2) calc(var(--spacing) * 4);
}
```

### `theme()`

```css:tailwind.css(Tailwind_v4)
.my-element {
  padding: theme(spacing.2) theme(spacing.4);
}
```

```css:output.css(compiled)
.my-element {
  /* The same as the output of the `--spacing()` function */
  padding: calc(var(--spacing) * 2) calc(var(--spacing) * 4);
}
```

## Dynamic Utility Values [^8]

In Tailwind v4, you can use casual values in your utilities. Tailwind v4 will automatically generate the corresponding CSS:

```tsx:my-component.tsx(Tailwind_v3)
const MyComponent = () => {
  return (
    <div class="opacity-[0.98] p-[18px]">
      {/* ... */}
    </div>
  );
};
```

```tsx:my-component.tsx(Tailwind_v4)
const MyComponent = () => {
  return (
    <div class="opacity-98 p-4.5">
      {/* ... */}
    </div>
  );
};
```

## Animation

In Tailwind v3, there is a popular plugin called [tailwindcss-animate](https://github.com/jamiebuilds/tailwindcss-animate) that provides a set of animation classes for Tailwind CSS.

```js:tailwind.config.mjs(Tailwind_v3)
/** @type {import('tailwindcss').Config} */
module.exports = {
  // ...
  plugins: [
    require('tailwindcss-animate'),
    // ...
  ],
}
```

In Tailwind v4, we can use [tw-animate-css](https://github.com/Wombosvideo/tw-animate-css):

```css:tailwind.css(Tailwind_v4)
@import 'tw-animate-css';
```

Usage:

```tsx:my-component.tsx
const MyComponent = () => {
  return (
    <div class="animate-in fade-in">
      {/* ... */}
    </div>
  );
};
```

## Wrapping Up

Tailwind v4's January release initially caused some confusion in the community, but as the most popular styling solution in the front-end ecosystem, these issues will likely be resolved soon.

Happy styling!

## Footnotes

[^1]: [Setting Your Base Path | Tailwind CSS v4](https://tailwindcss.com/docs/detecting-classes-in-source-files#setting-your-base-path)

[^2]: [Disabling Automatic Detection | Tailwind CSS v4](https://tailwindcss.com/docs/detecting-classes-in-source-files#disabling-automatic-detection)

[^3]: [Explicitly Excluding Classes | Tailwind CSS v4](https://tailwindcss.com/docs/detecting-classes-in-source-files#explicitly-excluding-classes)

[^4]: [Referencing Other Variables | Tailwind CSS v4](https://tailwindcss.com/docs/theme#referencing-other-variables)

[^5]: [Background Image | Tailwind CSS v4](https://tailwindcss.com/docs/background-image)

[^6]: [Utility Directive | Tailwind CSS v4](https://tailwindcss.com/docs/functions-and-directives#utility-directive)

[^7]: [Functions | Tailwind CSS v4](https://tailwindcss.com/docs/functions-and-directives#functions)

[^8]: [Dynamic Utility Values and Variants | Tailwind CSS v4](https://tailwindcss.com/blog/tailwindcss-v4#dynamic-utility-values-and-variants)
