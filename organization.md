> full path: `tree -f -I "node_modules"`
❯ tree -I "node_modules"
.
├── LICENSE
├── README.md
├── organization.md
├── rust
│   ├── graphql-mongo
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   ├── grpc-mongo
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── main.rs
│   └── shared
│       ├── Cargo.toml
│       └── src
│           └── lib.rs
├── typescript
│   ├── common
│   │   ├── @packages
│   │   │   ├── components
│   │   │   │   ├── mobile
│   │   │   │   ├── shared
│   │   │   │   └── web
│   │   │   │       └── Button.tsx
│   │   │   ├── hooks
│   │   │   │   ├── mobile
│   │   │   │   ├── shared
│   │   │   │   └── web
│   │   │   └── utils
│   │   │       ├── mobile
│   │   │       ├── shared
│   │   │       │   └── getShared.ts
│   │   │       └── web
│   │   ├── App.tsx
│   │   ├── README.md
│   │   ├── app.json
│   │   ├── assets
│   │   │   ├── fonts
│   │   │   │   └── SpaceMono-Regular.ttf
│   │   │   └── images
│   │   │       ├── adaptive-icon.png
│   │   │       ├── favicon.png
│   │   │       ├── icon.png
│   │   │       └── splash.png
│   │   ├── babel.config.js
│   │   ├── package-lock.json
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── types.tsx
│   ├── copy-packages.sh
│   ├── frontend-main
│   │   ├── @packages
│   │   │   ├── components
│   │   │   ├── hooks
│   │   │   └── utils
│   │   │       └── getShared.ts
│   │   ├── README.md
│   │   ├── next-env.d.ts
│   │   ├── next.config.js
│   │   ├── package-lock.json
│   │   ├── package.json
│   │   ├── pages
│   │   │   ├── _app.tsx
│   │   │   ├── api
│   │   │   │   └── hello.ts
│   │   │   └── index.tsx
│   │   ├── public
│   │   │   └── favicon.ico
│   │   ├── styles
│   │   │   └── globals.css
│   │   └── tsconfig.json
│   ├── install-packages-for-all.sh
│   └── mobile-main
│       ├── @packages
│       │   ├── components
│       │   ├── hooks
│       │   └── utils
│       │       └── getShared.ts
│       ├── App.tsx
│       ├── app.json
│       ├── assets
│       │   ├── fonts
│       │   │   └── SpaceMono-Regular.ttf
│       │   └── images
│       │       ├── adaptive-icon.png
│       │       ├── favicon.png
│       │       ├── icon.png
│       │       └── splash.png
│       ├── babel.config.js
│       ├── components
│       │   ├── EditScreenInfo.tsx
│       │   ├── StyledText.tsx
│       │   ├── Themed.tsx
│       │   └── __tests__
│       │       └── StyledText-test.js
│       ├── constants
│       │   ├── Colors.ts
│       │   └── Layout.ts
│       ├── hooks
│       │   ├── useCachedResources.ts
│       │   └── useColorScheme.ts
│       ├── navigation
│       │   ├── LinkingConfiguration.ts
│       │   └── index.tsx
│       ├── package-lock.json
│       ├── package.json
│       ├── screens
│       │   ├── ModalScreen.tsx
│       │   ├── NotFoundScreen.tsx
│       │   ├── TabOneScreen.tsx
│       │   └── TabTwoScreen.tsx
│       ├── tsconfig.json
│       ├── types.tsx
│       └── yarn.lock
└── usful.md

48 directories, 67 files