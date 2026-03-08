# UserGrow Frontend

**Live Demo:** https://usergrow-two.vercel.app

## Overview

Complete single-page web application for UserGrow — an AI Brand Visibility Intelligence platform that analyzes how AI models (ChatGPT, Gemini, Perplexity) perceive brands across different personas, languages, and demographics.

## Features Implemented

### 🎨 Design
- **Glassmorphism dark theme** with frosted glass cards
- **Particle background** animation (interactive canvas)
- **Purple accent** (#6c5ce7) with cyan highlights (#18ffff)
- **Smooth animations**: counter animations, fade-in on scroll, tab transitions
- **Inter font** throughout
- **Fully responsive** mobile design

### 📊 6-Tab Analysis Dashboard

#### Tab 1: AI Visibility Score
- Large animated counter (0→score)
- Color-coded status (red/orange/green based on score)
- Chart.js horizontal bar chart showing "Share of Model"
- Competitor ranking bars showing who AI recommends instead

#### Tab 2: Persona Heatmap
- 6 personas × 6 model/language combinations (36 data points)
- Color-coded cells: green (top 3), yellow (4-6), red (7+), gray (not mentioned)
- Shows rank numbers with tooltips
- Reveals demographic bias patterns

#### Tab 3: Brand DNA
- CSS word cloud visualization with variable font sizes
- Sentiment color coding (green/red/gray)
- Side panel comparing "Your DNA vs Competitors"
- Shows keyword associations AI makes with your brand

#### Tab 4: Language Gap
- 3 language cards (English, Chinese, Spanish)
- Clear ✅/❌ indicators for visibility
- Shows dramatic differences in AI recommendations across languages

#### Tab 5: Reality Check
- Large "AI Bias Score" number
- Two-column comparison: "What AI Says" vs "Real World Data"
- Highlights mismatches in red
- Data source attribution (Healthgrades, Zocdoc, Yelp via Tavily)

#### Tab 6: Recommendations
- Numbered action items (1-6)
- Priority badges: 🔴 Critical, 🟡 Important, 🟢 Nice-to-have
- Detailed explanations for each recommendation
- Actionable steps to improve AI visibility

### 🔄 User Flow

1. **Search Page (Hero)**
   - Brand name input
   - Industry dropdown (Healthcare, Finance, Legal, Restaurant, SaaS, Education, Retail, Other)
   - City input (optional)
   - "🔍 Analyze" button

2. **Loading Screen**
   - 7 animated progress steps
   - Each step shows spinner → ✅ completion
   - Steps: persona probes, AI probing (GPT/Gemini/Perplexity), analysis, real-world data, memory consultation

3. **Report Dashboard**
   - Sticky tab navigation
   - 6 tabs with comprehensive analysis
   - "← New Analysis" button to return to search

### 📦 Mock Data

Currently uses realistic mock data for **Dr. Smith Cardiology** (NYC healthcare example):
- Visibility score: 23/100 (Critical)
- Strong performance with English-speaking professionals
- **Invisible** to Spanish speakers and retirees (key demographics for cardiology!)
- Competitors: Mount Sinai, NYU Langone, NewYork-Presbyterian
- Real-world rating: 4.9★ but only 8% AI mention rate (huge gap)

## Technical Stack

- **Vanilla JavaScript** (no framework)
- **Chart.js** for bar charts
- **D3.js** included (available for future enhancements)
- **CSS animations** (keyframes, transitions)
- **Canvas API** for particle background
- **Mobile responsive** (flexbox/grid)

## File Structure

```
docs/
├── index.html      # Main HTML structure (all 3 pages)
├── styles.css      # Complete styling (21KB)
├── app.js          # All JavaScript logic (18KB)
└── README.md       # This file
```

## Research Foundation

The platform is grounded in academic research:
- **arXiv 2025**: "Revealing Potential Biases in LLM-Based Recommender Systems"
- **ACL 2025**: Systematic review validating demographic persona probing
- **Columbia/NYU**: "LLM Generated Persona is a Promise with a Catch"
- **US Census Bureau**: Demographic distributions for persona generation

## Development Status

✅ **Complete Features:**
- All 6 analysis tabs
- Loading screen with progress animation
- Sticky tab navigation
- Mock data for all visualizations
- Chart.js integration
- Word cloud DNA visualization
- Fully responsive design
- Particle background animation

🔜 **Future Integration:**
- Backend API connection (`POST /api/analyze`)
- Real Engram memory integration
- D3.js force-directed graph for Brand DNA
- Live data from Tavily API
- User accounts and historical tracking

## Deployment

**Production URL:** https://usergrow-two.vercel.app

Deployed via Vercel with automatic GitHub integration.

To redeploy:
```bash
cd /Users/potato/.openclaw/workspace-hackathon/docs
vercel --yes --prod
```

## Local Development

Simply open `index.html` in a browser:
```bash
open index.html
```

Or use a local server:
```bash
python3 -m http.server 8000
# Visit http://localhost:8000
```

## Brand Count Footer

Shows: "🧠 Agent has analyzed 247 brands | Powered by Engram Memory"

This can be dynamically updated when backend is connected.

---

**Built with 🦀 Rust + 🧠 Engram**

GitHub: https://github.com/luofang34/FDHackathon
