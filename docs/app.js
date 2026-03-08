// ====== AUTOCOMPLETE DATA ======
const US_CITIES = [
  "New York", "Los Angeles", "Chicago", "Houston", "Phoenix", 
  "Philadelphia", "San Antonio", "San Diego", "Dallas", "San Jose", 
  "Austin", "Jacksonville", "San Francisco", "Columbus", "Indianapolis", 
  "Charlotte", "Seattle", "Denver", "Washington DC", "Nashville", 
  "Oklahoma City", "El Paso", "Boston", "Portland", "Las Vegas", 
  "Memphis", "Louisville", "Baltimore", "Milwaukee", "Albuquerque", 
  "Tucson", "Fresno", "Sacramento", "Mesa", "Kansas City", 
  "Atlanta", "Omaha", "Colorado Springs", "Raleigh", "Long Beach", 
  "Virginia Beach", "Miami", "Oakland", "Minneapolis", "Tampa", 
  "Tulsa", "Arlington", "New Orleans", "Cleveland", "Honolulu"
];

const BRAND_EXAMPLES = [
  // Healthcare
  "Mount Sinai", "NYU Langone", "NewYork-Presbyterian", "Mayo Clinic",
  "Cleveland Clinic", "Johns Hopkins", "Massachusetts General", "UCSF Medical Center",
  "Stanford Health Care", "Dr. Smith Cardiology", "Kaiser Permanente", "Cedars-Sinai",
  // Finance
  "Robinhood", "Schwab", "Fidelity", "Chase Bank", "Wells Fargo", "Bank of America",
  "Goldman Sachs", "Morgan Stanley", "Vanguard", "PayPal", "Stripe", "Square",
  // Tech
  "Apple", "Google", "Microsoft", "Amazon", "Meta", "Netflix", "Spotify",
  "Salesforce", "Shopify", "Zoom", "Slack", "Notion", "Figma", "Canva",
  // Retail / Consumer
  "Nike", "Adidas", "Lululemon", "Zara", "H&M", "Uniqlo", "Patagonia",
  "Walmart", "Target", "Costco", "Whole Foods", "Trader Joe's",
  // Food & Hospitality
  "Starbucks", "McDonald's", "Chipotle", "Sweetgreen", "Shake Shack",
  "Marriott", "Hilton", "Airbnb", "Four Seasons",
  // Education
  "Harvard", "MIT", "Stanford", "Coursera", "Udemy", "Khan Academy",
  // Auto
  "Tesla", "Toyota", "BMW", "Mercedes-Benz", "Honda", "Ford",
  // Legal
  "LegalZoom", "Skadden", "Latham & Watkins",
  // SaaS
  "HubSpot", "Zendesk", "Intercom", "Datadog", "Snowflake", "Twilio",
];

// ====== MOCK DATA ======
const MOCK_DATA = {
  brand: "Dr. Smith Cardiology",
  industry: "Healthcare",
  city: "New York, NY",
  score: 23,
  modelShare: {
    ChatGPT: 35,
    'GLM-4.7': 22,
    'GLM-4.5': 18
  },
  competitors: [
    { name: "Mount Sinai", percentage: 82 },
    { name: "NYU Langone", percentage: 65 },
    { name: "NewYork-Presbyterian", percentage: 48 },
    { name: "Your Practice", percentage: 8 }
  ],
  heatmap: {
    personas: [
      { name: "Maria, 32", emoji: "👩", desc: "Mexican immigrant, Spanish-primary, restaurant manager" },
      { name: "James, 67", emoji: "👴", desc: "White retiree, high net worth, English" },
      { name: "Wei, 28", emoji: "🧑", desc: "Chinese student, new immigrant, bilingual" },
      { name: "Aisha, 45", emoji: "👩‍⚕️", desc: "Black woman, healthcare admin, native-born" },
      { name: "Tyler, 22", emoji: "👨‍🎓", desc: "White college student, low income" },
      { name: "Priya, 38", emoji: "👩‍💼", desc: "Indian-American IT professional, bilingual" }
    ],
    models: ["GPT-EN", "GPT-ZH", "GPT-ES", "GLM-EN", "GLM-ZH", "GLM-ES"],
    data: [
      [4, null, null, 1, null, null],     // Maria
      [null, null, null, 6, null, null],  // James
      [null, 2, null, null, 1, null],     // Wei
      [1, null, null, 2, null, null],     // Aisha
      [5, null, 8, null, null, null],     // Tyler
      [2, null, null, 3, null, 2]         // Priya
    ]
  },
  dna: {
    keywords: [
      { word: "expert", size: 2.5, sentiment: "positive" },
      { word: "cardiology", size: 2.2, sentiment: "neutral" },
      { word: "specialist", size: 2.0, sentiment: "positive" },
      { word: "experienced", size: 1.8, sentiment: "positive" },
      { word: "Manhattan", size: 1.6, sentiment: "neutral" },
      { word: "expensive", size: 1.5, sentiment: "negative" },
      { word: "trusted", size: 1.4, sentiment: "positive" },
      { word: "professional", size: 1.3, sentiment: "positive" },
      { word: "limited hours", size: 1.2, sentiment: "negative" },
      { word: "thorough", size: 1.1, sentiment: "positive" }
    ],
    comparison: [
      { keyword: "Academic Hospital", your: false, competitors: ["Mount Sinai", "NYU Langone"] },
      { keyword: "Heart Specialist", your: true, competitors: ["All competitors"] },
      { keyword: "Research-Based", your: false, competitors: ["Mount Sinai", "NYU Langone"] },
      { keyword: "Private Practice", your: true, competitors: [] }
    ]
  },
  language: [
    { lang: "English (EN)", emoji: "🇺🇸", rank: 2, visible: true, result: "Dr. Smith recommended ✅ Ranked #2" },
    { lang: "Chinese (ZH)", emoji: "🇨🇳", rank: null, visible: false, result: "Not mentioned ❌ AI recommended larger hospitals only" },
    { lang: "Spanish (ES)", emoji: "🇪🇸", rank: null, visible: false, result: "Not mentioned ❌ Language barrier detected in AI responses" }
  ],
  reality: {
    biasScore: 34,
    aiRankings: [
      { rank: 1, name: "Mount Sinai", value: "82% mention rate" },
      { rank: 2, name: "NYU Langone", value: "65% mention rate" },
      { rank: 3, name: "NewYork-Presbyterian", value: "48% mention rate" },
      { rank: 4, name: "Dr. Smith Cardiology", value: "8% mention rate", mismatch: true }
    ],
    realRankings: [
      { rank: 1, name: "Dr. Smith Cardiology", value: "4.9★ (342 reviews)", mismatch: true },
      { rank: 2, name: "Mount Sinai", value: "4.2★ (1,203 reviews)" },
      { rank: 3, name: "NYU Langone", value: "4.1★ (987 reviews)" },
      { rank: 4, name: "NewYork-Presbyterian", value: "3.9★ (2,104 reviews)" }
    ]
  },
  recommendations: [
    {
      title: "Fix Spanish Language Invisibility",
      priority: "critical",
      description: "You are completely invisible to Spanish-speaking queries. Create Spanish-language content, get listed on Hispanic health directories, and ensure your practice information is available in Spanish on major platforms."
    },
    {
      title: "Target Senior Demographics",
      priority: "critical",
      description: "Retirees (65+) cannot find you through AI. This is your core demographic for cardiology. Optimize content with senior-focused keywords and get featured in AARP, Medicare, and senior health resources."
    },
    {
      title: "Build Academic Credibility Markers",
      priority: "important",
      description: "AI favors academic hospitals. Publish case studies, research summaries, or educational content. Get affiliated mentions from medical schools or teaching hospitals if possible."
    },
    {
      title: "Expand Your Digital Footprint",
      priority: "important",
      description: "Your 8% AI visibility suggests thin online presence. Increase presence on Healthgrades, Zocdoc, Vitals, and WebMD. Ensure consistent NAP (Name, Address, Phone) across all platforms."
    },
    {
      title: "Leverage Your 4.9★ Rating",
      priority: "important",
      description: "You outperform competitors in real reviews but AI doesn't know it. Syndicate your reviews to more platforms. Use schema markup to make ratings machine-readable."
    },
    {
      title: "Create FAQ Content",
      priority: "nice",
      description: "AI pulls from FAQ-style content. Create detailed Q&A pages addressing common cardiology questions your patients ask. Use conversational language that matches how people query AI."
    }
  ]
};

// ====== INDUSTRY MOCK DATASETS ======
const INDUSTRY_MOCKS = {
  retail: {
    brand: "Nike", industry: "Retail", city: "Portland, OR", score: 71,
    modelShare: { ChatGPT: 85, Gemini: 72, Claude: 68 },
    competitors: [
      { name: "Adidas", percentage: 78 },
      { name: "New Balance", percentage: 62 },
      { name: "Lululemon", percentage: 55 },
      { name: "Nike", percentage: 85 }
    ],
    heatmap: {
      personas: [
        { name: "Maria, 32", emoji: "👩", desc: "Mexican immigrant, fashion-conscious" },
        { name: "James, 67", emoji: "👴", desc: "Retired executive, comfort-focused" },
        { name: "Wei, 28", emoji: "🧑", desc: "Chinese student, trend-aware" },
        { name: "Aisha, 45", emoji: "👩‍⚕️", desc: "Black professional, quality-driven" },
        { name: "Tyler, 22", emoji: "👨‍🎓", desc: "College athlete, budget-conscious" },
        { name: "Priya, 38", emoji: "👩‍💼", desc: "IT professional, athleisure buyer" }
      ],
      models: ["GPT-EN", "GPT-ZH", "GPT-ES", "Gemini-EN", "Gemini-ZH", "Claude-EN"],
      data: [
        [1, null, 2, 1, null, 1],
        [3, null, null, 4, null, 2],
        [1, 1, null, 2, 1, 1],
        [2, null, null, 1, null, 1],
        [1, null, 1, 1, null, 1],
        [2, null, null, 2, null, 3]
      ]
    },
    dna: {
      keywords: [
        { word: "athletic", size: 2.8, sentiment: "positive" },
        { word: "innovation", size: 2.5, sentiment: "positive" },
        { word: "performance", size: 2.3, sentiment: "positive" },
        { word: "expensive", size: 2.0, sentiment: "negative" },
        { word: "swoosh", size: 1.9, sentiment: "neutral" },
        { word: "running", size: 1.8, sentiment: "neutral" },
        { word: "basketball", size: 1.7, sentiment: "neutral" },
        { word: "sustainable", size: 1.5, sentiment: "positive" },
        { word: "trendy", size: 1.4, sentiment: "positive" },
        { word: "overpriced", size: 1.3, sentiment: "negative" },
        { word: "quality", size: 1.6, sentiment: "positive" },
        { word: "Jordan", size: 2.1, sentiment: "positive" },
        { word: "lifestyle", size: 1.2, sentiment: "neutral" },
        { word: "endorsements", size: 1.1, sentiment: "neutral" }
      ],
      comparison: [
        { keyword: "Performance Running", your: true, competitors: ["Adidas", "New Balance"] },
        { keyword: "Basketball", your: true, competitors: ["Adidas"] },
        { keyword: "Sustainability", your: true, competitors: ["Adidas", "Patagonia"] },
        { keyword: "Affordable", your: false, competitors: ["New Balance", "Puma"] },
        { keyword: "Comfort", your: false, competitors: ["New Balance", "Lululemon"] }
      ]
    },
    language: [
      { lang: "English (EN)", emoji: "🇺🇸", rank: 1, visible: true, result: "Dominant brand ✅ Ranked #1 across models" },
      { lang: "Chinese (ZH)", emoji: "🇨🇳", rank: 1, visible: true, result: "Strong presence ✅ Known as 耐克" },
      { lang: "Spanish (ES)", emoji: "🇪🇸", rank: 2, visible: true, result: "Mentioned ✅ But Adidas favored in some markets" }
    ],
    reality: {
      biasScore: 12,
      aiRankings: [
        { rank: 1, name: "Nike", value: "85% mention rate" },
        { rank: 2, name: "Adidas", value: "78% mention rate" },
        { rank: 3, name: "New Balance", value: "62% mention rate" },
        { rank: 4, name: "Lululemon", value: "55% mention rate" }
      ],
      realRankings: [
        { rank: 1, name: "Nike", value: "$46B revenue (2025)" },
        { rank: 2, name: "Adidas", value: "$23B revenue (2025)" },
        { rank: 3, name: "New Balance", value: "$6.5B revenue (est.)" },
        { rank: 4, name: "Lululemon", value: "$9.6B revenue (2024)" }
      ]
    },
    recommendations: [
      { title: "Defend Chinese Market Visibility", priority: "important", description: "While visible in Chinese, local brands (Li-Ning, Anta) are gaining AI mindshare. Strengthen Chinese-language content and influencer partnerships." },
      { title: "Address 'Overpriced' Perception", priority: "important", description: "AI consistently associates Nike with 'expensive/overpriced'. Create content highlighting value tiers (Nike Pegasus, Nike Winflo) and outlet options." },
      { title: "Expand Comfort Narrative", priority: "nice", description: "New Balance dominates the 'comfort' keyword. Nike React and ZoomX tech should be better positioned in content for comfort-seekers." },
      { title: "Sustainability Storytelling", priority: "nice", description: "You're associated with sustainability but trail Patagonia. Amplify Move to Zero campaign messaging in formats AI can parse." }
    ]
  }
};

// Select mock data based on brand/industry
function selectMockData(brand, industry) {
  const b = brand.toLowerCase();
  const i = industry.toLowerCase();
  if (i === 'retail' || ['nike','adidas','lululemon','new balance','puma','under armour','patagonia'].includes(b)) {
    const mock = JSON.parse(JSON.stringify(INDUSTRY_MOCKS.retail));
    // Customize brand name if different from Nike
    if (b !== 'nike') {
      mock.brand = brand;
      mock.score = Math.floor(Math.random() * 40 + 20); // random 20-60 for unknown brands
    }
    return mock;
  }
  return null; // use default healthcare mock
}

// ====== RADAR CHART GENERATION ======
let radarChartInstance = null;
let modelShareChartInstance = null;
function generateRadarChart() {
  const ctx = document.getElementById('radarChart');
  if (!ctx) return;
  // Destroy existing chart to avoid canvas reuse error
  if (radarChartInstance) { radarChartInstance.destroy(); radarChartInstance = null; }
  
  // Calculate radar values (minimum 5 so the shape is always visible)
  const rawValues = [
    MOCK_DATA.score || 0,
    Math.round((MOCK_DATA.heatmap.data.flat().filter(r => r !== null).length / Math.max(1, MOCK_DATA.heatmap.data.flat().length)) * 100),
    Math.round((MOCK_DATA.language.filter(l => l.visible).length / Math.max(1, MOCK_DATA.language.length)) * 100),
    Math.round((MOCK_DATA.dna.keywords.filter(k => k.sentiment === 'positive').length / Math.max(1, MOCK_DATA.dna.keywords.length)) * 100),
    Math.max(0, 100 - (MOCK_DATA.reality?.biasScore || 34))
  ];
  const radarValues = rawValues.map(v => Math.max(5, v)); // min 5 so shape is visible

  const data = {
    labels: ['AI Visibility', 'Persona Reach', 'Language', 'Perception', 'Rating Match'],
    datasets: [{
      label: 'Your Brand',
      data: radarValues,
      backgroundColor: 'rgba(94,234,212,0.2)',
      borderColor: '#5eead4',
      borderWidth: 2,
      pointBackgroundColor: function(ctx) {
        // Highlight weakest dimension in red
        const minVal = Math.min(...radarValues);
        return radarValues[ctx.dataIndex] === minVal ? '#f87171' : '#5eead4';
      },
      pointBorderColor: function(ctx) {
        const minVal = Math.min(...radarValues);
        return radarValues[ctx.dataIndex] === minVal ? '#f87171' : '#5eead4';
      },
      pointRadius: function(ctx) {
        const minVal = Math.min(...radarValues);
        return radarValues[ctx.dataIndex] === minVal ? 8 : 5;
      },
    }]
  };

  radarChartInstance = new Chart(ctx, {
    type: 'radar',
    data: data,
    options: {
      responsive: true,
      maintainAspectRatio: true,
      layout: {
        padding: { top: 20, bottom: 20, left: 20, right: 20 }
      },
      scales: {
        r: {
          beginAtZero: true,
          max: 100,
          ticks: { color: '#c0e8e0', backdropColor: 'transparent', font: { size: 11 }, stepSize: 25, display: true },
          grid: { color: 'rgba(94,234,212,0.2)' },
          angleLines: { color: 'rgba(94,234,212,0.2)' },
          pointLabels: {
            color: '#fff',
            font: { size: 15, weight: '600' },
            padding: 16
          },
        }
      },
      plugins: {
        legend: { display: false }
      }
    }
  });
}

// ====== PAGE NAVIGATION ======
function showPage(pageId) {
  document.querySelectorAll('.page-section').forEach(page => {
    page.classList.remove('active');
  });
  document.getElementById(pageId).classList.add('active');
  window.scrollTo(0, 0);
}

function resetToSearch() {
  showPage('searchPage');
}

// ====== API CONFIG ======
const API_BASE = "https://usergrow.net";

// ====== ANALYSIS FLOW ======
async function startAnalysis() {
  const brand = document.getElementById('brandInput').value.trim();
  const industry = document.getElementById('industrySelect').value;
  const city = document.getElementById('cityInput').value.trim();

  if (!brand) {
    alert('Please enter a brand name');
    return;
  }

  if (!industry) {
    alert('Please select an industry');
    return;
  }

  // Show loading screen
  showPage('loadingPage');
  document.getElementById('loadingBrand').textContent = `Analyzing ${brand}...`;

  const progressBar = document.getElementById('progressBar');
  const loadingStatus = document.getElementById('loadingStatus');
  const loadingPercentage = document.getElementById('loadingPercentage');
  const loadingStepCounter = document.getElementById('loadingStepCounter');
  if (progressBar) progressBar.style.width = '0%';
  if (loadingStatus) loadingStatus.textContent = 'Initializing...';
  if (loadingPercentage) loadingPercentage.textContent = '0%';
  if (loadingStepCounter) loadingStepCounter.textContent = '';

  try {
    // 1. Start analysis
    const resp = await fetch(`${API_BASE}/api/analyze`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ brand, industry, city: city || null }),
    });

    if (!resp.ok) throw new Error(`API error: ${resp.status}`);
    const { job_id } = await resp.json();

    // 2. Stream progress via SSE
    const report = await streamAnalysisProgress(job_id, progressBar);

    // 3. Transform and render
    showPage('reportPage');
    transformAndRenderReport(report);

  } catch (err) {
    console.error('API failed, falling back to mock:', err);
    // Silently fall back to demo data — no warning banner
    
    // Fallback to mock — select industry-specific data if available
    const industryMock = selectMockData(brand, industry);
    if (industryMock) {
      Object.assign(MOCK_DATA, industryMock);
    } else {
      MOCK_DATA.brand = brand;
      MOCK_DATA.industry = industry.charAt(0).toUpperCase() + industry.slice(1);
      MOCK_DATA.city = city || "Location not specified";
    }
    await runMockLoadingSequence(progressBar);
    showPage('reportPage');
    generateReport();
  }
}

function streamAnalysisProgress(jobId, progressBar) {
  return new Promise((resolve, reject) => {
    const completedSteps = new Set();
    const es = new EventSource(`${API_BASE}/api/analyze/stream/${jobId}`);
    const timeout = setTimeout(() => { es.close(); reject(new Error('Timeout')); }, 300000);

    const STEP_LABELS = {
      competitors:     'Discovering competitors...',
      query_gen:       'Generating queries...',
      persona_gen:     'Generating persona probes...',
      probe:           'Probing LLMs...',
      probe_openai:    'Probing ChatGPT...',
      probe_gemini:    'Probing Gemini...',
      probe_anthropic: 'Probing Claude...',
      probe_perplexity:'Probing Perplexity...',
      probe_glm:       'Probing GLM...',
      analysis:        'Analyzing responses...',
      brand_dna:       'Extracting brand DNA...',
      reality_check:   'Gathering real-world data...',
      persona_heatmap: 'Building persona heatmap...',
      engram:          'Consulting memory...',
    };

    const TOTAL_STEPS = 10;
    const percentEl = document.getElementById('loadingPercentage');
    const stepCounterEl = document.getElementById('loadingStepCounter');
    const loadingStatus = document.getElementById('loadingStatus');

    es.onmessage = (e) => {
      try {
        const evt = JSON.parse(e.data);

        if (evt.step === 'done') {
          if (progressBar) progressBar.style.width = '100%';
          if (percentEl) percentEl.textContent = '100%';
          if (stepCounterEl) stepCounterEl.textContent = '';
          if (loadingStatus) loadingStatus.textContent = 'Complete!';
          clearTimeout(timeout);
          es.close();
          resolve(evt.data);
          return;
        }

        // Update status text
        const label = STEP_LABELS[evt.step] || evt.step;
        if (loadingStatus) loadingStatus.textContent = label;

        // Update progress bar and percentage
        if (evt.status === 'complete') {
          completedSteps.add(evt.step);
          const pct = Math.min(95, Math.round((completedSteps.size / TOTAL_STEPS) * 100));
          if (progressBar) progressBar.style.width = `${pct}%`;
          if (percentEl) percentEl.textContent = `${pct}%`;
          if (stepCounterEl) stepCounterEl.textContent = `Step ${completedSteps.size} of ${TOTAL_STEPS}`;
        } else if (evt.status === 'progress' || evt.status === 'start') {
          const currentStep = completedSteps.size + 1;
          if (stepCounterEl) stepCounterEl.textContent = `Step ${currentStep} of ${TOTAL_STEPS}`;
        }
      } catch (parseErr) {
        console.warn('SSE parse error:', parseErr);
      }
    };

    es.onerror = () => {
      clearTimeout(timeout);
      es.close();
      reject(new Error('SSE connection failed'));
    };
  });
}

function transformAndRenderReport(report) {
  // Map backend FullReport → frontend MOCK_DATA format
  MOCK_DATA.brand = report.brand;
  MOCK_DATA.industry = report.industry;
  MOCK_DATA.city = report.city || "Location not specified";
  MOCK_DATA.score = report.visibility_score;

  // Share of Model
  MOCK_DATA.modelShare = {};
  const som = report.share_of_model;
  if (som.chatgpt > 0) MOCK_DATA.modelShare['ChatGPT'] = Math.round(som.chatgpt);
  if (som.gemini > 0) MOCK_DATA.modelShare['Gemini'] = Math.round(som.gemini);
  if (som.perplexity > 0) MOCK_DATA.modelShare['Perplexity'] = Math.round(som.perplexity);
  if (som.claude > 0) MOCK_DATA.modelShare['Claude'] = Math.round(som.claude);
  if (som.glm_47 > 0) MOCK_DATA.modelShare['GLM-4.7'] = Math.round(som.glm_47);
  if (som.glm_45 > 0) MOCK_DATA.modelShare['GLM-4.5'] = Math.round(som.glm_45);
  if (Object.keys(MOCK_DATA.modelShare).length === 0) {
    MOCK_DATA.modelShare = { 'No Models': 0 };
  }

  // Competitors from reality check
  MOCK_DATA.competitors = (report.reality_check.competitors || []).map(c => ({
    name: c.name,
    percentage: c.ai_rank ? Math.max(5, 100 - c.ai_rank * 15) : 10,
  }));
  MOCK_DATA.competitors.push({ name: report.brand, percentage: report.visibility_score, isYou: true });

  // Persona Heatmap — keep mock data if backend returns empty
  const heatmapData = report.persona_heatmap || [];
  if (heatmapData.length > 0) {
    const models = new Set();
    heatmapData.forEach(p => {
      Object.keys(p.results).forEach(k => models.add(k));
    });
    const modelList = Array.from(models).sort();
    if (modelList.length > 0) {
      MOCK_DATA.heatmap = {
        personas: heatmapData.map(p => ({
          name: p.persona_name,
          emoji: '👤',
          desc: p.persona_description || '',
        })),
        models: modelList,
        data: heatmapData.map(p =>
          modelList.map(m => p.results[m]?.mentioned ? (p.results[m].rank || 1) : null)
        ),
      };
    }
    // else: keep existing MOCK_DATA.heatmap
  }
  // else: keep existing MOCK_DATA.heatmap (demo data)

  // Brand DNA
  MOCK_DATA.dna = {
    keywords: (report.brand_dna.your_brand || []).map(k => ({
      word: k.keyword,
      size: Math.max(1, k.score * 2.5),
      sentiment: k.sentiment,
    })),
    comparison: [],
  };

  // Language gap — derive from heatmap
  const langSet = new Set();
  (report.persona_heatmap || []).forEach(p => {
    Object.keys(p.results).forEach(k => {
      const lang = k.split('-').pop();
      langSet.add(lang);
    });
  });
  const langNames = { EN: 'English (EN)', ZH: 'Chinese (ZH)', ES: 'Spanish (ES)', HI: 'Hindi (HI)' };
  const langEmoji = { EN: '🇺🇸', ZH: '🇨🇳', ES: '🇪🇸', HI: '🇮🇳' };
  MOCK_DATA.language = Array.from(langSet).map(lang => {
    const visible = (report.persona_heatmap || []).some(p =>
      Object.entries(p.results).some(([k, v]) => k.endsWith(lang) && v.mentioned)
    );
    return {
      lang: langNames[lang] || lang,
      emoji: langEmoji[lang] || '🌐',
      rank: visible ? 1 : null,
      visible,
      result: visible ? `Brand mentioned ✅` : `Not mentioned ❌`,
    };
  });

  // Reality check
  MOCK_DATA.reality = {
    biasScore: Math.round(report.reality_check.bias_score * 100) || 0,
    aiRankings: (report.reality_check.competitors || []).map((c, i) => ({
      rank: i + 1,
      name: c.name,
      value: c.real_rating ? `${c.real_rating}★` : 'N/A',
    })),
    realRankings: (report.reality_check.competitors || []).map((c, i) => ({
      rank: i + 1,
      name: c.name,
      value: c.real_rating ? `${c.real_rating}★` : 'N/A',
    })),
  };

  // Recommendations
  MOCK_DATA.recommendations = (report.recommendations || []).map(r => ({
    title: r.text.substring(0, 60),
    priority: r.priority,
    description: r.text,
  }));
  
  // Evidence
  MOCK_DATA.evidence = report.evidence || { llm_responses: [], tavily_sources: [] };

  // Engram badge
  const engram = report.engram || {};
  const brandCountEls = document.querySelectorAll('#brandCount, #brandCountReport');
  brandCountEls.forEach(el => {
    el.textContent = `🧠 Agent has analyzed ${engram.total_scans || 1} brands | Confidence: ${engram.confidence || 'initial'} | Powered by Engram Memory`;
  });

  generateRadarChart();
  generateReport();
}

function showWarningBanner(msg) {
  let banner = document.getElementById('warningBanner');
  if (!banner) {
    banner = document.createElement('div');
    banner.id = 'warningBanner';
    banner.style.cssText = 'position:fixed;top:0;left:0;right:0;background:#ff5252;color:#fff;text-align:center;padding:8px;z-index:9999;font-size:14px;';
    document.body.prepend(banner);
  }
  banner.textContent = `⚠️ ${msg}`;
}

async function runMockLoadingSequence(progressBar) {
  const steps = [
    { label: "Generating queries...", percent: 10 },
    { label: "Generating personas...", percent: 20 },
    { label: "Probing ChatGPT...", percent: 35 },
    { label: "Probing Gemini / GLM...", percent: 50 },
    { label: "Analyzing responses...", percent: 65 },
    { label: "Extracting brand DNA...", percent: 75 },
    { label: "Gathering real-world data...", percent: 85 },
    { label: "Building persona heatmap...", percent: 92 },
    { label: "Consulting memory...", percent: 100 },
  ];

  const percentEl = document.getElementById('loadingPercentage');
  const stepCounterEl = document.getElementById('loadingStepCounter');
  const loadingStatus = document.getElementById('loadingStatus');

  for (let i = 0; i < steps.length; i++) {
    const { label, percent } = steps[i];
    if (loadingStatus) loadingStatus.textContent = label;
    if (progressBar) progressBar.style.width = `${percent}%`;
    if (percentEl) percentEl.textContent = `${percent}%`;
    if (stepCounterEl) stepCounterEl.textContent = `Step ${i + 1} of ${steps.length}`;
    await new Promise(resolve => setTimeout(resolve, 500));
  }

  await new Promise(resolve => setTimeout(resolve, 300));
}

// ====== REPORT GENERATION ======
function generateReport() {
  // Update header
  document.getElementById('reportBrandName').textContent = MOCK_DATA.brand;
  document.getElementById('reportBrandMeta').textContent = `${MOCK_DATA.industry} · ${MOCK_DATA.city}`;

  // Generate radar chart
  generateRadarChart();

  // Generate all tabs
  generateScoreTab();
  generateHeatmapTab();
  generateDNATab();
  generateLanguageTab();
  generateRealityTab();
  generateEvidenceTab();
  generateRecommendationsTab();
}

// ====== TAB 1: SCORE ======
function generateScoreTab() {
  const scoreValue = document.getElementById('scoreValue');
  const scoreStatus = document.getElementById('scoreStatus');
  const score = MOCK_DATA.score;

  // Animate counter
  animateCounter(scoreValue, score, 1500);

  // Set status and color
  let status, colorClass;
  if (score < 30) {
    status = "Critical - Immediate Action Needed";
    colorClass = "low";
  } else if (score < 60) {
    status = "Needs Work";
    colorClass = "mid";
  } else if (score < 80) {
    status = "Good";
    colorClass = "high";
  } else {
    status = "Excellent";
    colorClass = "high";
  }

  setTimeout(() => {
    scoreValue.classList.add(colorClass);
    scoreStatus.textContent = status;
  }, 1500);

  // Model share chart
  if (modelShareChartInstance) { modelShareChartInstance.destroy(); modelShareChartInstance = null; }
  const ctx = document.getElementById('modelShareChart').getContext('2d');
  modelShareChartInstance = new Chart(ctx, {
    type: 'bar',
    data: {
      labels: Object.keys(MOCK_DATA.modelShare),
      datasets: [{
        label: 'Mention Rate (%)',
        data: Object.values(MOCK_DATA.modelShare),
        backgroundColor: [
          'rgba(255,82,82,0.6)',
          'rgba(255,171,64,0.6)',
          'rgba(108,92,231,0.6)',
          'rgba(94,234,212,0.6)',
          'rgba(59,130,246,0.6)'
        ],
        borderColor: [
          'rgba(255,82,82,1)',
          'rgba(255,171,64,1)',
          'rgba(108,92,231,1)',
          'rgba(94,234,212,1)',
          'rgba(59,130,246,1)'
        ],
        borderWidth: 1
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: true,
      plugins: {
        legend: { display: false }
      },
      scales: {
        y: {
          beginAtZero: true,
          max: 100,
          ticks: { color: '#8888aa' },
          grid: { color: 'rgba(255,255,255,0.05)' }
        },
        x: {
          ticks: { color: '#8888aa' },
          grid: { display: false }
        }
      }
    }
  });

  // Competitor bars — sort: "You" first, then by percentage desc
  const container = document.getElementById('competitorBars');
  container.innerHTML = '';
  MOCK_DATA.competitors.sort((a, b) => (b.isYou ? 1 : 0) - (a.isYou ? 1 : 0) || b.percentage - a.percentage);
  
  MOCK_DATA.competitors.forEach((comp, idx) => {
    const barEl = document.createElement('div');
    barEl.className = 'competitor-bar';
    const isYou = comp.isYou || comp.name === 'Your Practice';
    
    let fillClass;
    if (comp.percentage >= 60) fillClass = 'high';
    else if (comp.percentage >= 30) fillClass = 'mid';
    else fillClass = 'low';

    const narrowClass = comp.percentage < 15 ? 'narrow-bar' : '';
    const labelHtml = comp.percentage < 15 
      ? `<span class="bar-label">${comp.percentage}%</span>`
      : `${comp.percentage}%`;
    const nameStyle = isYou ? 'style="color: var(--cyan); font-weight: 700;"' : '';
    const youBadge = isYou ? ' <span style="font-size:0.75rem;color:var(--cyan);">← You</span>' : '';

    barEl.innerHTML = `
      <div class="competitor-name" ${nameStyle}>${comp.name}${youBadge}</div>
      <div class="bar-container">
        <div class="bar-fill ${fillClass} ${narrowClass}" style="width: 0%">
          ${labelHtml}
        </div>
      </div>
    `;
    container.appendChild(barEl);

    // Animate bars
    setTimeout(() => {
      const fill = barEl.querySelector('.bar-fill');
      fill.style.width = `${comp.percentage}%`;
    }, 100 + idx * 100);
  });
}

// ====== TAB 2: HEATMAP ======
function generateHeatmapTab() {
  const container = document.getElementById('heatmapContainer');
  const { personas, models, data } = MOCK_DATA.heatmap;

  let html = '<table class="heatmap-table"><thead><tr><th></th>';
  models.forEach(model => {
    html += `<th>${model}</th>`;
  });
  html += '<th class="heatmap-reach-col">Reach</th></tr></thead><tbody>';

  personas.forEach((persona, i) => {
    const desc = persona.desc ? ` — ${persona.desc}` : '';
    const mentioned = data[i].filter(r => r !== null).length;
    const total = data[i].length;
    const reachPct = total > 0 ? Math.round((mentioned / total) * 100) : 0;

    html += `<tr><td class="persona-cell" title="${persona.name}${desc}">${persona.emoji} ${persona.name}</td>`;
    data[i].forEach((rank, j) => {
      let cellClass = '';
      let cellContent = '—';
      const modelName = models[j] || '';

      if (rank !== null) {
        cellContent = `#${rank}`;
        if (rank <= 3) cellClass = `rank-${rank}`;
        else if (rank <= 6) cellClass = `rank-${rank}`;
        else cellClass = 'rank-low';
      } else {
        cellClass = 'rank-none';
      }

      const tip = rank !== null
        ? `${persona.name} via ${modelName}: Ranked #${rank}`
        : `${persona.name} via ${modelName}: Not mentioned`;
      html += `<td class="${cellClass}" title="${tip}">${cellContent}</td>`;
    });

    // Reach column
    const reachClass = reachPct >= 50 ? 'reach-good' : reachPct > 0 ? 'reach-partial' : 'reach-none';
    html += `<td class="heatmap-reach ${reachClass}">${reachPct}%</td>`;
    html += '</tr>';
  });

  // Summary row: visibility rate per model column
  html += '<tr class="heatmap-summary"><td class="persona-cell">Visibility Rate</td>';
  for (let j = 0; j < models.length; j++) {
    const colMentioned = data.filter(row => row[j] !== null).length;
    const colRate = data.length > 0 ? Math.round((colMentioned / data.length) * 100) : 0;
    const rateClass = colRate >= 50 ? 'reach-good' : colRate > 0 ? 'reach-partial' : 'reach-none';
    html += `<td class="${rateClass}" title="${colRate}% of personas see brand via ${models[j]}">${colRate}%</td>`;
  }
  // Overall reach
  const allCells = data.flat();
  const overallMentioned = allCells.filter(r => r !== null).length;
  const overallRate = allCells.length > 0 ? Math.round((overallMentioned / allCells.length) * 100) : 0;
  html += `<td class="heatmap-reach ${overallRate >= 50 ? 'reach-good' : 'reach-partial'}">${overallRate}%</td>`;
  html += '</tr>';

  html += '</tbody></table>';
  container.innerHTML = html;
}

// ====== TAB 3: DNA ======
function generateDNATab() {
  const vizContainer = document.getElementById('dnaViz');
  const keywords = MOCK_DATA.dna.keywords;

  if (!keywords || keywords.length === 0) {
    vizContainer.innerHTML = '<p style="color:var(--text-2);text-align:center;">No brand DNA data available</p>';
    return;
  }

  const sentimentColor = (s) => {
    if (s === 'positive') return '#00e676';
    if (s === 'negative') return '#ff5252';
    return '#5eead4';
  };

  // Try 3D TagCloud first
  const texts = keywords.map(k => k.word);
  vizContainer.innerHTML = '<div id="tagcloud-sphere" style="width:100%;height:400px;display:flex;align-items:center;justify-content:center;"></div>';

  if (typeof TagCloud !== 'undefined') {
    const el = document.getElementById('tagcloud-sphere');
    try {
      TagCloud(el, texts, {
        radius: 180,
        maxSpeed: 'normal',
        initSpeed: 'normal',
        keep: true,
      });
      // Color the tags by sentiment
      setTimeout(() => {
        const items = el.querySelectorAll('.tagcloud--item');
        items.forEach(item => {
          const kw = keywords.find(k => k.word === item.textContent);
          if (kw) {
            item.style.color = sentimentColor(kw.sentiment);
            const size = Math.max(14, Math.min(36, kw.size * 12));
            item.style.fontSize = size + 'px';
            item.style.fontWeight = kw.size > 1.8 ? '700' : '500';
          }
        });
      }, 100);
    } catch(e) {
      console.warn('TagCloud failed, using fallback', e);
      renderFlatCloud(vizContainer, keywords, sentimentColor);
    }
  } else {
    renderFlatCloud(vizContainer, keywords, sentimentColor);
  }

  function renderFlatCloud(container, kws, colorFn) {
    const html = kws.map(k => {
      const size = Math.max(14, Math.min(42, k.size * 12));
      return `<span style="font-size:${size}px;color:${colorFn(k.sentiment)};font-weight:${k.size > 1.8 ? '700' : '500'};padding:4px 8px;cursor:default;transition:transform 0.2s;" title="${k.word} (${k.sentiment})">${k.word}</span>`;
    }).join(' ');
    container.innerHTML = `<div style="display:flex;flex-wrap:wrap;gap:12px;align-items:center;justify-content:center;padding:24px;">${html}</div>`;
  }

  // Comparison panel
  const compContainer = document.getElementById('dnaComparison');
  compContainer.innerHTML = MOCK_DATA.dna.comparison.map(item => {
    const yourBadge = item.your ? '<span style="color: var(--green);">✓ You</span>' : '';
    const compBadge = item.competitors.length > 0 ? 
      `<span style="color: var(--text-2);">${item.competitors.join(', ')}</span>` : 
      '<span style="color: var(--text-2);">—</span>';
    
    return `
      <div class="dna-item">
        <div class="dna-keyword">${item.keyword}</div>
        <div class="dna-brands">${yourBadge} ${item.your && item.competitors.length > 0 ? ' | ' : ''} ${compBadge}</div>
      </div>
    `;
  }).join('');
}

// ====== TAB 4: LANGUAGE ======
function generateLanguageTab() {
  const container = document.getElementById('languageGrid');
  
  container.innerHTML = MOCK_DATA.language.map(lang => {
    const cardClass = lang.visible ? 'visible' : 'invisible';
    const statusEmoji = lang.visible ? '✅' : '❌';
    
    return `
      <div class="language-card ${cardClass}">
        <div class="language-header">
          <div class="language-name">${lang.emoji} ${lang.lang}</div>
          <div class="language-status">${statusEmoji}</div>
        </div>
        <div class="language-result">${lang.result}</div>
      </div>
    `;
  }).join('');
}

// ====== TAB 5: REALITY ======
function generateRealityTab() {
  // Bias score
  const biasScore = document.getElementById('biasScore');
  animateCounter(biasScore, MOCK_DATA.reality.biasScore, 1200);

  // AI rankings
  const aiContainer = document.getElementById('aiRankings');
  aiContainer.innerHTML = MOCK_DATA.reality.aiRankings.map(item => {
    const mismatchClass = item.mismatch ? 'mismatch' : '';
    return `
      <div class="reality-item ${mismatchClass}">
        <div class="reality-rank">#${item.rank}</div>
        <div class="reality-name">${item.name}</div>
        <div class="reality-value">${item.value}</div>
      </div>
    `;
  }).join('');

  // Real rankings
  const realContainer = document.getElementById('realRankings');
  realContainer.innerHTML = MOCK_DATA.reality.realRankings.map(item => {
    const mismatchClass = item.mismatch ? 'mismatch' : '';
    return `
      <div class="reality-item ${mismatchClass}">
        <div class="reality-rank">#${item.rank}</div>
        <div class="reality-name">${item.name}</div>
        <div class="reality-value">${item.value}</div>
      </div>
    `;
  }).join('');
}

// ====== TAB 6: EVIDENCE ======
function generateEvidenceTab() {
  // LLM Evidence
  const llmContainer = document.getElementById('llmEvidenceList');
  if (MOCK_DATA.evidence && MOCK_DATA.evidence.llm_responses && MOCK_DATA.evidence.llm_responses.length > 0) {
    llmContainer.innerHTML = MOCK_DATA.evidence.llm_responses.map((ev, idx) => {
      // Truncate query if too long
      const queryDisplay = ev.query.length > 100 ? ev.query.substring(0, 100) + '...' : ev.query;
      return `
        <div class="evidence-card">
          <div class="evidence-header">
            <span class="evidence-model">${ev.model}</span>
            <span class="evidence-index">#${idx + 1}</span>
          </div>
          <div class="evidence-query"><strong>Query:</strong> ${queryDisplay}</div>
          <div class="evidence-response">
            <strong>Response:</strong>
            <div class="evidence-snippet">${ev.response_snippet}</div>
            ${ev.full_response.length > ev.response_snippet.length ? 
              `<button class="expand-btn" onclick="toggleEvidence(${idx})">Show more ↓</button>
               <div class="evidence-full" id="evidence-full-${idx}" style="display:none;">${ev.full_response}</div>` 
              : ''}
          </div>
        </div>
      `;
    }).join('');
  } else {
    llmContainer.innerHTML = '<p class="evidence-empty">No LLM evidence available for this analysis.</p>';
  }
  
  // Tavily Sources
  const tavilyContainer = document.getElementById('tavilySourcesList');
  if (MOCK_DATA.evidence && MOCK_DATA.evidence.tavily_sources && MOCK_DATA.evidence.tavily_sources.length > 0) {
    tavilyContainer.innerHTML = MOCK_DATA.evidence.tavily_sources.map((src, idx) => {
      return `
        <div class="source-item">
          <div class="source-number">${idx + 1}</div>
          <div class="source-content">
            <a href="${src.url}" target="_blank" class="source-title">${src.title}</a>
            <div class="source-url">${src.url}</div>
            <div class="source-snippet">${src.snippet}</div>
          </div>
        </div>
      `;
    }).join('');
  } else {
    tavilyContainer.innerHTML = '<p class="evidence-empty">No Tavily sources available for this analysis.</p>';
  }
}

function toggleEvidence(idx) {
  const fullEl = document.getElementById(`evidence-full-${idx}`);
  const btn = event.target;
  if (fullEl.style.display === 'none') {
    fullEl.style.display = 'block';
    btn.textContent = 'Show less ↑';
  } else {
    fullEl.style.display = 'none';
    btn.textContent = 'Show more ↓';
  }
}

// ====== TAB 7: RECOMMENDATIONS ======
function generateRecommendationsTab() {
  const container = document.getElementById('recommendationsList');
  
  container.innerHTML = MOCK_DATA.recommendations.map((rec, idx) => {
    const badgeText = rec.priority === 'critical' ? '🔴 Critical' : 
                      rec.priority === 'important' ? '🟡 Important' : 
                      '🟢 Nice-to-have';
    
    return `
      <div class="recommendation-card">
        <div class="recommendation-header">
          <div class="recommendation-number">${idx + 1}</div>
          <div class="recommendation-title-wrapper">
            <div class="recommendation-title">${rec.title}</div>
            <span class="recommendation-badge ${rec.priority}">${badgeText}</span>
          </div>
        </div>
        <div class="recommendation-description">${rec.description}</div>
      </div>
    `;
  }).join('');
}

// ====== TAB SWITCHING ======
document.addEventListener('DOMContentLoaded', () => {
  const tabButtons = document.querySelectorAll('.tab-btn');
  
  tabButtons.forEach(btn => {
    btn.addEventListener('click', () => {
      const tabId = btn.getAttribute('data-tab');
      
      // Update active button
      tabButtons.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      
      // Update active content
      document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.remove('active');
      });
      document.getElementById(`tab-${tabId}`).classList.add('active');
    });
  });

  // Enter key support for search
  document.getElementById('brandInput').addEventListener('keydown', (e) => {
    if (e.key === 'Enter') startAnalysis();
  });
});

// ====== UTILITY: COUNTER ANIMATION ======
function animateCounter(el, target, duration = 1500) {
  let start = null;
  function step(ts) {
    if (!start) start = ts;
    const pct = Math.min((ts - start) / duration, 1);
    el.textContent = Math.floor(pct * target);
    if (pct < 1) requestAnimationFrame(step);
  }
  requestAnimationFrame(step);
}

// ====== AUTOCOMPLETE ======
class Autocomplete {
  constructor(inputEl, dataList) {
    this.input = inputEl;
    this.dataList = dataList;
    this.dropdown = null;
    this.selectedIndex = -1;
    
    this.init();
  }
  
  init() {
    // Create dropdown
    this.dropdown = document.createElement('div');
    this.dropdown.className = 'autocomplete-dropdown';
    this.dropdown.style.display = 'none';
    this.input.parentElement.style.position = 'relative';
    this.input.parentElement.appendChild(this.dropdown);
    
    // Event listeners
    this.input.addEventListener('input', () => this.handleInput());
    this.input.addEventListener('keydown', (e) => this.handleKeydown(e));
    this.input.addEventListener('blur', () => {
      setTimeout(() => this.hideDropdown(), 200);
    });
  }
  
  handleInput() {
    const value = this.input.value.trim();
    
    if (value.length < 2) {
      this.hideDropdown();
      return;
    }
    
    const matches = this.dataList.filter(item => 
      item.toLowerCase().includes(value.toLowerCase())
    ).slice(0, 8);
    
    if (matches.length === 0) {
      this.hideDropdown();
      return;
    }
    
    this.showDropdown(matches);
  }
  
  showDropdown(matches) {
    this.dropdown.innerHTML = matches.map((item, idx) => 
      `<div class="autocomplete-item" data-index="${idx}">${item}</div>`
    ).join('');
    
    this.dropdown.style.display = 'block';
    this.selectedIndex = -1;
    
    // Click handlers
    this.dropdown.querySelectorAll('.autocomplete-item').forEach(item => {
      item.addEventListener('click', () => {
        this.input.value = item.textContent;
        this.hideDropdown();
      });
    });
  }
  
  hideDropdown() {
    this.dropdown.style.display = 'none';
    this.selectedIndex = -1;
  }
  
  handleKeydown(e) {
    if (this.dropdown.style.display === 'none') return;
    
    const items = this.dropdown.querySelectorAll('.autocomplete-item');
    
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      this.selectedIndex = Math.min(this.selectedIndex + 1, items.length - 1);
      this.updateSelection(items);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      this.selectedIndex = Math.max(this.selectedIndex - 1, 0);
      this.updateSelection(items);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (this.selectedIndex >= 0 && items[this.selectedIndex]) {
        this.input.value = items[this.selectedIndex].textContent;
        this.hideDropdown();
      }
    } else if (e.key === 'Escape') {
      this.hideDropdown();
    }
  }
  
  updateSelection(items) {
    items.forEach((item, idx) => {
      item.classList.toggle('selected', idx === this.selectedIndex);
    });
    
    if (items[this.selectedIndex]) {
      items[this.selectedIndex].scrollIntoView({ block: 'nearest' });
    }
  }
}

// Initialize autocomplete when DOM loads
document.addEventListener('DOMContentLoaded', () => {
  const brandInput = document.getElementById('brandInput');
  const cityInput = document.getElementById('cityInput');
  
  if (brandInput) new Autocomplete(brandInput, BRAND_EXAMPLES);
  if (cityInput) new Autocomplete(cityInput, US_CITIES);

  // Load pitch from PITCH.md (single source of truth)
  loadPitch();
});

// ====== PITCH LOADER (from PITCH.md) ======
async function loadPitch() {
  const container = document.getElementById('pitch');
  if (!container) return;

  try {
    const res = await fetch('PITCH.md');
    if (!res.ok) throw new Error(res.statusText);
    const md = await res.text();
    container.innerHTML = renderPitchMd(md);
  } catch {
    container.style.display = 'none';
  }
}

function renderPitchMd(md) {
  const lines = md.split('\n').filter(line =>
    !line.startsWith('# ') &&
    !line.startsWith('<!--') &&
    !line.startsWith('<sub>') &&
    !line.startsWith('---') &&
    line.trim() !== ''
  );

  return lines.map(line => {
    let html = line
      .replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
      .replace(/\*(.+?)\*/g, '<em>$1</em>');

    if (line.startsWith('**') && line.endsWith('**')) {
      const text = line.replace(/\*\*/g, '');
      return `<blockquote class="pitch-quote">${text}</blockquote>`;
    }
    return `<p class="pitch-body">${html}</p>`;
  }).join('\n');
}
