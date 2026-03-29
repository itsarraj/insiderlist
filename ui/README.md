# MatchMyResume - SEO-Optimized React SPA Template

A modern, SEO-friendly React Single Page Application template built with TypeScript, Vite, Tailwind CSS, and DaisyUI. Features pre-rendering for excellent search engine optimization without Next.js or SSR.

## 🚀 Features

### SEO Optimization
- **Pre-rendering** with react-snap for static HTML generation
- **Dynamic meta tags** with react-helmet-async
- **Automatic sitemap generation**
- **Structured data (JSON-LD)** for rich search results
- **Semantic HTML** structure
- **robots.txt** and **web app manifest**

### Modern Stack
- ⚡ **Vite** - Lightning fast build tool
- ⚛️ **React 19** with TypeScript
- 🎨 **Tailwind CSS** + **DaisyUI** components
- 🧭 **React Router** for SPA navigation
- 🔐 **Universal Auth Service** - Complete authentication system
- 🔐 **Authentication Pages** - Login, Register, Reset Password
- 🔤 **Hanken Grotesk Font** - Custom typography throughout
- 🔍 **SEO-first** architecture

## 🏗️ Architecture

```
src/
├── components/     # Reusable UI components
│   ├── Header.tsx
│   └── Footer.tsx
├── pages/         # Route components with SEO
│   ├── HomePage.tsx
│   ├── AboutPage.tsx
│   ├── ContactPage.tsx
│   ├── LoginPage.tsx
│   ├── RegisterPage.tsx
│   ├── ResetPasswordPage.tsx
│   └── NotFoundPage.tsx
├── App.tsx        # Main app with routing
├── main.tsx       # Entry point
└── index.css      # Global styles + Tailwind
```

## 🔐 Authentication Features

### Complete Auth Flow
- **Login Page** (`/login`) - Email/password with social login (GitHub, LinkedIn, Google)
- **Register Page** (`/register`) - Full registration with validation
- **Reset Password** (`/reset-password`) - Password recovery flow

### Features
- ✅ **Form Validation** - Client-side validation with error messages
- ✅ **Loading States** - Visual feedback during form submission
- ✅ **Social Login** - GitHub, LinkedIn, Google integration ready
- ✅ **Responsive Design** - Mobile-first approach
- ✅ **SEO Optimized** - Each page has proper meta tags
- ✅ **Brand Consistent** - Uses your `#2b2b2b` background and `#fcc636` accent colors

### Styling
- Dark theme with brand colors (`#2b2b2b`, `#fcc636`)
- **Hanken Grotesk** as the primary font family
- Consistent shadows and rounded corners
- Hover effects and transitions
- Mobile-responsive layouts

### Typography
**Hanken Grotesk Font Family:**
- **Variable Fonts**: Modern browser support with full weight range (100-900)
- **Static Fallbacks**: 18 weights from Thin to Black (regular & italic)
- **Auto-fallback**: Graceful degradation for older browsers
- **Font Loading**: `font-display: swap` for optimal performance

Font files are located in `public/fonts/` and automatically loaded via CSS.

## 🔐 Authentication System

### Universal Auth Service Integration

**Complete Authentication Flow:**
- **Registration** with email verification
- **Login/Logout** with JWT tokens
- **Password Reset** via email
- **Token Refresh** for session management
- **Profile Management** for authenticated users

### API Endpoints
```
POST /api/v1/auth/register       # User registration
POST /api/v1/auth/login          # User login
POST /api/v1/auth/refresh        # Token refresh
POST /api/v1/auth/logout         # User logout
GET  /api/v1/auth/verify-email   # Email verification
POST /api/v1/auth/password-reset # Request password reset
POST /api/v1/auth/password-reset/confirm # Reset password
GET  /api/v1/user/profile        # Get user profile
PUT  /api/v1/user/profile        # Update profile
POST /api/v1/user/change-password # Change password
```

### Security Features
- **JWT Tokens**: Access + Refresh token system
- **Token Blacklisting**: Secure logout and token revocation
- **Rate Limiting**: Protection against brute force attacks
- **Email Verification**: Account security
- **Password Hashing**: Bcrypt with salt
- **CORS Protection**: Cross-origin request security

### Frontend Architecture
```
contexts/
├── AuthContext.tsx     # Authentication state management

services/
├── authApi.ts         # API client for auth endpoints

components/
├── ProtectedRoute.tsx # Route protection component
```

### Authentication States
- **Loading**: Checking authentication status
- **Authenticated**: User logged in with valid tokens
- **Unauthenticated**: User not logged in
- **Error**: Authentication failures

### Token Management
- **Local Storage**: Secure token storage
- **Auto-refresh**: Automatic token renewal
- **Cleanup**: Secure logout and data clearing
- **Error Handling**: Graceful failure recovery

## 🛠️ Getting Started

### Prerequisites
- Node.js 18+
- npm or yarn

### Installation

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Start development server:**
   ```bash
   npm run dev
   ```

3. **Build for production:**
   ```bash
   npm run build
   ```

4. **Build with SEO pre-rendering:**
   ```bash
   npm run build:seo
   ```

## 🎯 SEO Features Explained

### 1. Pre-rendering (No SSR Required)

This template uses **react-snap** to pre-render your React SPA into static HTML files. Search engines see fully rendered content instead of empty `<div id="root">`.

**How it works:**
1. Build your React app normally
2. react-snap launches a headless browser
3. Crawls all routes and saves rendered HTML
4. Result: Static HTML files that search engines love

**Generated structure:**
```
dist/
├── index.html          # Pre-rendered homepage
├── about/index.html    # Pre-rendered about page
├── contact/index.html  # Pre-rendered contact page
└── sitemap.xml         # Auto-generated sitemap
```

### 2. Dynamic Meta Tags

Each page uses **react-helmet-async** for SEO-optimized meta tags:

```tsx
import { Helmet } from 'react-helmet-async'

function HomePage() {
  return (
    <>
      <Helmet>
        <title>MatchMyResume - AI-Powered Resume Job Matching</title>
        <meta name="description" content="Find your dream job..." />
        <meta property="og:title" content="MatchMyResume..." />
        <meta property="og:description" content="Find your dream job..." />
        <meta name="twitter:card" content="summary_large_image" />
      </Helmet>
      {/* Page content */}
    </>
  )
}
```

### 3. Structured Data (Rich Results)

JSON-LD schema markup for Google rich results:

```tsx
<Helmet>
  <script type="application/ld+json">
    {JSON.stringify({
      "@context": "https://schema.org",
      "@type": "WebApplication",
      "name": "MatchMyResume",
      "description": "AI-powered resume matching platform"
    })}
  </script>
</Helmet>
```

### 4. Automatic Sitemap Generation

Runs after each build to create `sitemap.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://matchmyresume.com/</loc>
    <lastmod>2024-01-15</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <!-- More URLs -->
</urlset>
```

## 🎨 Styling

### DaisyUI Components

Built-in DaisyUI themes and components:

```tsx
// Button examples
<button className="btn btn-primary">Primary</button>
<button className="btn btn-secondary">Secondary</button>

// Cards
<div className="card bg-base-100 shadow-xl">
  <div className="card-body">
    <h2 className="card-title">Card Title</h2>
    <p>Card content</p>
  </div>
</div>

// Navbar
<nav className="navbar bg-base-100">
  <div className="navbar-start">...</div>
  <div className="navbar-center">...</div>
  <div className="navbar-end">...</div>
</nav>
```

### Custom Tailwind Classes

```css
/* src/index.css */
@layer components {
  .btn-primary {
    @apply btn btn-primary;
  }
}
```

## 🚀 Deployment

### Static Hosting (Recommended)

Deploy to any static host - the pre-rendered HTML gives you excellent SEO:

- **Vercel** - `npm run build:seo`
- **Netlify** - `npm run build:seo`
- **Cloudflare Pages** - `npm run build:seo`
- **GitHub Pages** - `npm run build:seo`

### SEO Performance Comparison

| Method | SEO Score | Speed | Complexity |
|--------|-----------|-------|------------|
| This Template | ⭐⭐⭐⭐⭐ | ⚡ | 🟢 Simple |
| Next.js SSR | ⭐⭐⭐⭐⭐ | 🐌 | 🔴 Complex |
| Plain React SPA | ⭐⭐ | ⚡ | 🟢 Simple |

## 📝 Scripts

```bash
# Development
npm run dev          # Start dev server
npm run preview      # Preview production build

# Building
npm run build        # Build for production
npm run build:seo    # Build + pre-render for SEO

# SEO Tools
npm run generate-sitemap  # Generate sitemap manually

# Code Quality
npm run lint         # Run ESLint
```

## 🔧 Customization

### Adding New Routes

1. **Create page component** in `src/pages/`
2. **Add route** in `App.tsx`
3. **Update sitemap script** in `scripts/generate-sitemap.js`
4. **Add to react-snap config** in `package.json`

### SEO Configuration

- **Update base URL** in `scripts/generate-sitemap.js`
- **Modify meta tags** in each page component
- **Add new routes** to react-snap config

### Styling Themes

DaisyUI supports 30+ themes. Change in `tailwind.config.js`:

```js
daisyui: {
  themes: ["light", "dark", "dracula", "corporate"],
}
```

## 🔍 SEO Best Practices Included

✅ **Semantic HTML** structure
✅ **Proper heading hierarchy** (H1 → H2 → H3)
✅ **Alt text** for images
✅ **Descriptive link text**
✅ **Mobile-first responsive design**
✅ **Fast loading** with code splitting
✅ **Clean URLs** with React Router
✅ **404 page** with proper status codes

## 🚀 Performance Optimizations

- **Code splitting** by route
- **Lazy loading** for components
- **Optimized bundles** with Vite
- **CSS purging** with Tailwind
- **Pre-compressed assets**

## 🤝 Contributing

1. Follow the existing code structure
2. Add SEO meta tags to new pages
3. Update sitemap when adding routes
4. Test pre-rendering with `npm run build:seo`

## 📄 License

ISC License

---

## Why This Approach Works

**Traditional React SPA Problem:**
```html
<body>
  <div id="root"></div>
  <script src="bundle.js"></script>
</body>
```
*Search engines see nothing*

**This Template Solution:**
```html
<body>
  <div id="root">
    <h1>MatchMyResume</h1>
    <p>AI-powered resume matching...</p>
    <!-- Fully rendered content -->
  </div>
</body>
```
*Search engines see everything*

**Result:** Excellent SEO without SSR complexity! 🎯