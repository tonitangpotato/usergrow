// ====== UserGrow Knowledge Graph — Layered Visibility Map ======

const KG_COLORS = {
  brand:    '#00e5a0',
  keyword:  '#ff6b6b',
  persona:  '#4ecdc4',
  model:    '#ffe66d',
  industry: '#a29bfe',
  city:     '#fd79a8',
};

const KG_SIZES = {
  brand: 14, keyword: 5, persona: 9, model: 16, industry: 26, city: 12,
};

// Score → color (red→orange→yellow→green)
function scoreColor(score) {
  if (score == null) return '#666';
  const s = Math.max(0, Math.min(20, score));
  if (s <= 5)  return d3.interpolateRgb('#ef4444', '#f97316')(s / 5);
  if (s <= 10) return d3.interpolateRgb('#f97316', '#eab308')((s - 5) / 5);
  if (s <= 15) return d3.interpolateRgb('#eab308', '#22c55e')((s - 10) / 5);
  return d3.interpolateRgb('#22c55e', '#00e5a0')((s - 15) / 5);
}

function scoreLabel(score) {
  if (score == null) return 'No data';
  if (score <= 3)  return 'Invisible';
  if (score <= 7)  return 'Low';
  if (score <= 12) return 'Moderate';
  if (score <= 16) return 'Visible';
  return 'Dominant';
}

let kgSimulation, kgSvg, kgG, kgLink, kgNode, kgLabel;
let kgAllData = { entities: [], relations: [] };
let kgZoom;

// Active layers
let kgLayers = { models: false, competition: false, keywords: false };

async function kgLoadGraph() {
  try {
    const resp = await fetch('/api/kg/graph');
    const data = await resp.json();
    kgAllData = data;
    document.getElementById('kgLoading').classList.add('hidden');

    // Populate industry filter
    const industries = [...new Set(
      data.entities.filter(e => e.entity_type === 'industry').map(e => e.name)
    )].sort();
    const sel = document.getElementById('kgIndustryFilter');
    industries.forEach(ind => {
      const opt = document.createElement('option');
      opt.value = ind;
      opt.textContent = ind.charAt(0).toUpperCase() + ind.slice(1);
      sel.appendChild(opt);
    });
    if (industries.length === 1) sel.value = industries[0];

    kgApplyFilters();
    kgPopulateStats(data);
  } catch (err) {
    document.getElementById('kgLoading').innerHTML =
      `<span style="color:#ff6b6b">Failed to load: ${err.message}</span>`;
  }
}

function kgUpdateStats(data) {
  document.getElementById('kgStatEntities').textContent = data.entities.length;
  document.getElementById('kgStatRelations').textContent = data.relations.length;
}

function kgRenderGraph(data) {
  const container = document.getElementById('kgContainer');
  const width = container.clientWidth;
  const height = container.clientHeight;

  if (kgSimulation) kgSimulation.stop();
  d3.select('#kgSvg').selectAll('*').remove();

  if (data.entities.length === 0) {
    d3.select('#kgSvg').append('text')
      .attr('x', width/2).attr('y', height/2)
      .attr('text-anchor', 'middle').attr('fill', '#666').attr('font-size', '14px')
      .text('No data — run an analysis to populate the graph');
    return;
  }

  kgSvg = d3.select('#kgSvg').attr('viewBox', [0, 0, width, height]);
  kgZoom = d3.zoom().scaleExtent([0.1, 6]).on('zoom', e => kgG.attr('transform', e.transform));
  kgSvg.call(kgZoom);
  kgG = kgSvg.append('g');

  // Parse metadata for scores
  data.entities.forEach(e => {
    if (e.metadata && typeof e.metadata === 'string') {
      try { e.metadata = JSON.parse(e.metadata); } catch {}
    }
  });

  const nodeMap = {};
  const nodes = data.entities.map(e => {
    const n = { ...e, x: width/2 + (Math.random()-0.5)*400, y: height/2 + (Math.random()-0.5)*400 };
    nodeMap[e.id] = n;
    return n;
  });

  const links = data.relations
    .filter(r => nodeMap[r.source_id] && nodeMap[r.target_id])
    .map(r => ({ source: r.source_id, target: r.target_id, relation_type: r.relation_type, weight: r.weight }));

  // Degree for sizing
  const degree = {};
  links.forEach(l => {
    const s = typeof l.source === 'string' ? l.source : l.source.id;
    const t = typeof l.target === 'string' ? l.target : l.target.id;
    degree[s] = (degree[s] || 0) + 1;
    degree[t] = (degree[t] || 0) + 1;
  });

  function nodeRadius(d) {
    if (d.entity_type === 'brand' && d.metadata?.visibility_score != null) {
      return 8 + (d.metadata.visibility_score / 20) * 16; // 8-24 based on score
    }
    const base = KG_SIZES[d.entity_type] || 8;
    return base + Math.min((degree[d.id] || 0) * 0.3, 6);
  }

  function nodeColor(d) {
    if (d.entity_type === 'brand') {
      return scoreColor(d.metadata?.visibility_score);
    }
    return KG_COLORS[d.entity_type] || '#888';
  }

  const n = nodes.length;
  const brands = nodes.filter(nd => nd.entity_type === 'brand');
  const isSingleIndustry = nodes.filter(nd => nd.entity_type === 'industry').length <= 1 && brands.length > 10;

  kgSimulation = d3.forceSimulation(nodes)
    .force('link', d3.forceLink(links).id(d => d.id)
      .distance(d => {
        if (d.relation_type === 'belongs_to') return isSingleIndustry ? 280 : 200;
        if (d.relation_type === 'competes_with') return 180;
        if (d.relation_type === 'visible_in') return 200;
        if (d.relation_type === 'associated_with') return 120;
        return 150;
      })
      .strength(d => {
        if (d.relation_type === 'belongs_to') return 0.05;
        if (d.relation_type === 'associated_with') return 0.1;
        return Math.min(d.weight || 0.15, 0.25);
      }))
    .force('charge', d3.forceManyBody().strength(d => {
      if (d.entity_type === 'industry') return -2500;
      if (d.entity_type === 'model') return -1200;
      if (d.entity_type === 'brand') return isSingleIndustry ? -900 : -500;
      return -200;
    }))
    .force('center', d3.forceCenter(width / 2, height / 2).strength(0.03))
    .force('collision', d3.forceCollide().radius(d => nodeRadius(d) + 14).strength(0.85))
    .force('x', d3.forceX(width / 2).strength(0.02))
    .force('y', d3.forceY(height / 2).strength(0.02));

  // Glow filter
  const defs = kgSvg.append('defs');
  const glow = defs.append('filter').attr('id', 'kgGlow');
  glow.append('feGaussianBlur').attr('stdDeviation', '3').attr('result', 'blur');
  glow.append('feMerge').selectAll('feMergeNode').data(['blur', 'SourceGraphic']).join('feMergeNode').attr('in', d => d);

  // Relation color based on type
  function linkColor(d) {
    const colors = {
      visible_in:      'rgba(255, 230, 109, 0.4)',
      visible_to:      'rgba(78, 205, 196, 0.35)',
      competes_with:   'rgba(255, 107, 107, 0.5)',
      associated_with: 'rgba(255, 107, 107, 0.15)',
      belongs_to:      'rgba(162, 155, 254, 0.2)',
      located_in:      'rgba(253, 121, 168, 0.2)',
    };
    return colors[d.relation_type] || 'rgba(255,255,255,0.06)';
  }

  // Links
  kgLink = kgG.append('g')
    .selectAll('line').data(links).join('line')
    .attr('stroke', linkColor)
    .attr('stroke-width', d => {
      if (d.relation_type === 'competes_with') return 1.5;
      return Math.max(0.5, (d.weight || 0.2) * 2);
    })
    .attr('stroke-dasharray', d => d.relation_type === 'competes_with' ? '5,3' : null);

  // Nodes
  kgNode = kgG.append('g')
    .selectAll('circle').data(nodes).join('circle')
    .attr('r', nodeRadius)
    .attr('fill', nodeColor)
    .attr('stroke', d => {
      if (d.entity_type === 'brand') return d3.color(nodeColor(d)).brighter(0.8);
      return d3.color(KG_COLORS[d.entity_type] || '#888').brighter(0.5);
    })
    .attr('stroke-width', d => d.entity_type === 'industry' ? 2.5 : 1.2)
    .attr('filter', d => ['brand', 'industry'].includes(d.entity_type) ? 'url(#kgGlow)' : null)
    .attr('opacity', d => d.entity_type === 'keyword' ? 0.65 : 1)
    .attr('cursor', 'pointer')
    .on('mouseover', kgShowTooltip)
    .on('mousemove', kgMoveTooltip)
    .on('mouseout', kgHideTooltip)
    .on('click', kgHighlight)
    .call(d3.drag()
      .on('start', (e, d) => { if (!e.active) kgSimulation.alphaTarget(0.3).restart(); d.fx = d.x; d.fy = d.y; })
      .on('drag', (e, d) => { d.fx = e.x; d.fy = e.y; })
      .on('end', (e, d) => { if (!e.active) kgSimulation.alphaTarget(0); d.fx = null; d.fy = null; }));

  // Labels
  kgLabel = kgG.append('g')
    .selectAll('text')
    .data(nodes.filter(nd => {
      if (nd.entity_type === 'industry') return true;
      if (nd.entity_type === 'model') return true;
      if (nd.entity_type === 'brand') return (degree[nd.id] || 0) >= 2 || n < 80;
      return false;
    }))
    .join('text')
    .attr('text-anchor', 'middle')
    .attr('dy', d => nodeRadius(d) + 14)
    .attr('font-size', d => d.entity_type === 'industry' ? '11px' : '8px')
    .attr('fill', d => {
      if (d.entity_type === 'brand') return d3.color(nodeColor(d)).brighter(0.5);
      return d3.color(KG_COLORS[d.entity_type] || '#888').brighter(0.3);
    })
    .attr('font-weight', d => ['brand', 'industry'].includes(d.entity_type) ? '600' : '400')
    .attr('font-family', "'DM Sans', sans-serif")
    .attr('paint-order', 'stroke')
    .attr('stroke', 'rgba(10,14,23,0.8)')
    .attr('stroke-width', 2.5)
    .text(d => d.name.length > 22 ? d.name.substring(0, 20) + '…' : d.name);

  kgSimulation.on('tick', () => {
    kgLink.attr('x1', d => d.source.x).attr('y1', d => d.source.y)
          .attr('x2', d => d.target.x).attr('y2', d => d.target.y);
    kgNode.attr('cx', d => d.x).attr('cy', d => d.y);
    kgLabel.attr('x', d => d.x).attr('y', d => d.y);
  });

  setTimeout(() => kgZoomToFit(width, height), 2000);
  setTimeout(() => kgZoomToFit(width, height), 4000);
}

function kgZoomToFit(width, height) {
  if (!kgG || !kgSvg || !kgZoom) return;
  const nodes = kgG.selectAll('circle').data();
  if (!nodes.length) return;
  let x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
  nodes.forEach(d => { x0 = Math.min(x0, d.x); y0 = Math.min(y0, d.y); x1 = Math.max(x1, d.x); y1 = Math.max(y1, d.y); });
  const pad = 80;
  x0 -= pad; y0 -= pad; x1 += pad; y1 += pad;
  const scale = Math.min(0.95, Math.min(width / (x1 - x0), height / (y1 - y0)));
  const cx = (x0 + x1) / 2, cy = (y0 + y1) / 2;
  kgSvg.transition().duration(800).call(
    kgZoom.transform, d3.zoomIdentity.translate(width/2, height/2).scale(scale).translate(-cx, -cy)
  );
}

// Tooltip
function kgShowTooltip(event, d) {
  const tip = document.getElementById('kgTooltip');
  const color = d.entity_type === 'brand' ? scoreColor(d.metadata?.visibility_score) : (KG_COLORS[d.entity_type] || '#888');
  const connections = kgAllData.relations.filter(r => r.source_id === d.id || r.target_id === d.id).length;

  let extra = '';
  if (d.entity_type === 'brand' && d.metadata) {
    const s = d.metadata.visibility_score;
    extra = `
      <div style="margin-top:6px;font-size:0.85rem;">
        <div>Score: <strong style="color:${scoreColor(s)}">${s ?? '—'}/20</strong> — ${scoreLabel(s)}</div>
        ${d.metadata.chatgpt_pct != null ? `<div style="margin-top:3px;color:#aaa">GPT: ${d.metadata.chatgpt_pct}% · Claude: ${d.metadata.claude_pct}% · Gemini: ${d.metadata.gemini_pct}%</div>` : ''}
      </div>
    `;
  }

  tip.innerHTML = `
    <span class="kg-type" style="background:${color}33;color:${color}">${d.entity_type}</span>
    <h3>${d.name}</h3>
    <div class="kg-meta">${connections} connection${connections !== 1 ? 's' : ''}</div>
    ${extra}
  `;
  tip.classList.add('visible');
  tip.style.left = event.clientX + 15 + 'px';
  tip.style.top = event.clientY - 10 + 'px';
}

function kgMoveTooltip(event) {
  const tip = document.getElementById('kgTooltip');
  tip.style.left = event.clientX + 15 + 'px';
  tip.style.top = event.clientY - 10 + 'px';
}

function kgHideTooltip() {
  document.getElementById('kgTooltip').classList.remove('visible');
}

function kgHighlight(event, d) {
  const connected = new Set([d.id]);
  kgAllData.relations.forEach(r => {
    if (r.source_id === d.id) connected.add(r.target_id);
    if (r.target_id === d.id) connected.add(r.source_id);
  });
  kgNode.attr('opacity', n => connected.has(n.id) ? 1 : 0.06);
  kgLabel.attr('opacity', n => connected.has(n.id) ? 1 : 0.06);
  kgLink.attr('opacity', l => {
    const src = typeof l.source === 'object' ? l.source.id : l.source;
    const tgt = typeof l.target === 'object' ? l.target.id : l.target;
    return (src === d.id || tgt === d.id) ? 1 : 0.02;
  });
  event.stopPropagation();
}

document.getElementById('kgSvg').addEventListener('click', (e) => {
  if (e.target.tagName === 'svg' || e.target.tagName === 'SVG') {
    if (kgNode) kgNode.attr('opacity', d => d.entity_type === 'keyword' ? 0.65 : 1);
    if (kgLabel) kgLabel.attr('opacity', 1);
    if (kgLink) kgLink.attr('opacity', 1);
  }
});

function kgResetZoom() {
  if (kgSvg && kgZoom) kgSvg.transition().duration(600).call(kgZoom.transform, d3.zoomIdentity);
}

// Layer toggles
document.querySelectorAll('.kg-layer-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const layer = btn.dataset.layer;
    kgLayers[layer] = !kgLayers[layer];
    btn.classList.toggle('active', kgLayers[layer]);
    kgApplyFilters();
  });
});

// Filters
document.getElementById('kgIndustryFilter').addEventListener('change', kgApplyFilters);

function kgApplyFilters() {
  const indFilter = document.getElementById('kgIndustryFilter').value;

  let filtered = { entities: [...kgAllData.entities], relations: [...kgAllData.relations] };

  // Industry filter
  if (indFilter !== 'all') {
    const indEntity = kgAllData.entities.find(e => e.entity_type === 'industry' && e.name === indFilter);
    if (indEntity) {
      const brandIds = new Set();
      kgAllData.relations.forEach(r => {
        if (r.target_id === indEntity.id && r.relation_type === 'belongs_to') brandIds.add(r.source_id);
        if (r.source_id === indEntity.id && r.relation_type === 'belongs_to') brandIds.add(r.target_id);
      });
      const showIds = new Set([indEntity.id, ...brandIds]);
      // Always include connected entities based on active layers
      kgAllData.relations.forEach(r => {
        if (brandIds.has(r.source_id)) showIds.add(r.target_id);
        if (brandIds.has(r.target_id)) showIds.add(r.source_id);
      });
      filtered.entities = kgAllData.entities.filter(e => showIds.has(e.id));
      filtered.relations = kgAllData.relations.filter(r => showIds.has(r.source_id) && showIds.has(r.target_id));
    }
  }

  // Apply layer visibility — base layer always shows brands + industry
  const visibleTypes = new Set(['brand', 'industry']);
  const visibleRelTypes = new Set(['belongs_to', 'competes_with']); // always show structure

  if (kgLayers.models) {
    visibleTypes.add('model');
    visibleRelTypes.add('visible_in');
  }
  if (kgLayers.competition) {
    visibleRelTypes.add('competes_with');
  }
  if (kgLayers.keywords) {
    visibleTypes.add('keyword');
    visibleRelTypes.add('associated_with');
  }

  // Personas layer could be added later
  // Always show competes_with in base
  visibleRelTypes.add('competes_with');

  // Filter entities by visible types
  const typeFilteredEntities = filtered.entities.filter(e => visibleTypes.has(e.entity_type));
  const typeFilteredIds = new Set(typeFilteredEntities.map(e => e.id));
  const typeFilteredRelations = filtered.relations.filter(r =>
    visibleRelTypes.has(r.relation_type) && typeFilteredIds.has(r.source_id) && typeFilteredIds.has(r.target_id)
  );

  kgRenderGraph({ entities: typeFilteredEntities, relations: typeFilteredRelations });
  kgUpdateStats({ entities: typeFilteredEntities, relations: typeFilteredRelations });
}

// ====== Data Stats ======

function kgPopulateStats(data) {
  const statsEl = document.getElementById('kgDataStats');
  
  // Count brands per industry
  const industryEntities = {};
  data.entities.forEach(e => {
    if (e.entity_type === 'industry') industryEntities[e.id] = e.name;
  });
  
  const brandsByIndustry = {};
  const analyzedBrands = new Set(); // brands with score = actually analyzed
  data.relations.forEach(r => {
    if (r.relation_type === 'belongs_to' && industryEntities[r.target_id]) {
      const ind = industryEntities[r.target_id];
      if (!brandsByIndustry[ind]) brandsByIndustry[ind] = new Set();
      brandsByIndustry[ind].add(r.source_id);
    }
  });
  
  // Count analyzed brands (those with visibility_score in metadata)
  data.entities.forEach(e => {
    if (e.entity_type === 'brand' && e.metadata?.visibility_score != null) {
      analyzedBrands.add(e.id);
    }
  });

  const totalBrands = data.entities.filter(e => e.entity_type === 'brand').length;
  const totalKeywords = data.entities.filter(e => e.entity_type === 'keyword').length;
  const totalRelations = data.relations.length;
  const numIndustries = Object.keys(brandsByIndustry).length;

  // Build industry tags
  const industryTags = Object.entries(brandsByIndustry)
    .sort((a, b) => b[1].size - a[1].size)
    .map(([name, ids]) => {
      const analyzed = [...ids].filter(id => analyzedBrands.has(id)).length;
      const label = name.charAt(0).toUpperCase() + name.slice(1);
      return `<span class="kg-stat-industry-tag">${label} <span class="kg-stat-industry-count">${analyzed}</span> brands</span>`;
    }).join('');

  statsEl.innerHTML = `
    <div class="kg-stat-card">
      <div class="kg-stat-number">${analyzedBrands.size}</div>
      <div class="kg-stat-label">Brands Analyzed</div>
      <div class="kg-stat-detail">across ${numIndustries} industries</div>
    </div>
    <div class="kg-stat-card">
      <div class="kg-stat-number">${totalKeywords}</div>
      <div class="kg-stat-label">Keywords Tracked</div>
      <div class="kg-stat-detail">extracted from AI responses</div>
    </div>
    <div class="kg-stat-card">
      <div class="kg-stat-number">${totalRelations.toLocaleString()}</div>
      <div class="kg-stat-label">Relations Mapped</div>
      <div class="kg-stat-detail">competition · visibility · keywords</div>
    </div>
    <div class="kg-stat-card" style="min-width:auto;">
      <div class="kg-stat-number">5</div>
      <div class="kg-stat-label">AI Models Probed</div>
      <div class="kg-stat-detail">GPT · Claude · Gemini · GLM</div>
    </div>
  `;

  // Add industry breakdown below stats
  if (industryTags) {
    const tagRow = document.createElement('div');
    tagRow.style.cssText = 'display:flex;justify-content:center;gap:10px;flex-wrap:wrap;margin-top:-12px;margin-bottom:20px;';
    tagRow.innerHTML = industryTags;
    statsEl.parentNode.insertBefore(tagRow, statsEl.nextSibling);
  }
}

// ====== Brand Search ======

const kgSearchInput = document.getElementById('kgSearchInput');
const kgSearchResults = document.getElementById('kgSearchResults');

kgSearchInput.addEventListener('input', () => {
  const q = kgSearchInput.value.trim().toLowerCase();
  if (q.length < 1) {
    kgSearchResults.classList.remove('open');
    return;
  }
  const brands = kgAllData.entities
    .filter(e => e.entity_type === 'brand' && e.name.toLowerCase().includes(q))
    .slice(0, 8);

  if (brands.length === 0) {
    kgSearchResults.innerHTML = '<div class="kg-search-item" style="color:var(--text-2);pointer-events:none;">No brands found</div>';
  } else {
    kgSearchResults.innerHTML = brands.map(b => {
      const score = b.metadata?.visibility_score;
      const color = scoreColor(score);
      return `<div class="kg-search-item" data-id="${b.id}">
        <span style="width:10px;height:10px;border-radius:50%;background:${color};flex-shrink:0;display:inline-block;"></span>
        ${b.name}
        <span class="kg-search-score" style="background:${color}22;color:${color}">${score ?? '—'}/20</span>
      </div>`;
    }).join('');
  }
  kgSearchResults.classList.add('open');
});

kgSearchResults.addEventListener('click', (e) => {
  const item = e.target.closest('.kg-search-item');
  if (!item || !item.dataset.id) return;
  kgSearchInput.value = '';
  kgSearchResults.classList.remove('open');
  kgFocusNode(item.dataset.id);
});

// Close dropdown when clicking outside
document.addEventListener('click', (e) => {
  if (!e.target.closest('.kg-search-wrap')) {
    kgSearchResults.classList.remove('open');
  }
});

function kgFocusNode(nodeId) {
  if (!kgG || !kgSvg || !kgZoom) return;
  const nodes = kgG.selectAll('circle').data();
  const target = nodes.find(d => d.id === nodeId);
  if (!target) return;

  const container = document.getElementById('kgContainer');
  const width = container.clientWidth;
  const height = container.clientHeight;

  // Zoom to node
  const scale = 2.0;
  kgSvg.transition().duration(800).call(
    kgZoom.transform,
    d3.zoomIdentity.translate(width/2, height/2).scale(scale).translate(-target.x, -target.y)
  );

  // Highlight connected nodes
  const connected = new Set([nodeId]);
  kgAllData.relations.forEach(r => {
    if (r.source_id === nodeId) connected.add(r.target_id);
    if (r.target_id === nodeId) connected.add(r.source_id);
  });
  kgNode.attr('opacity', n => connected.has(n.id) ? 1 : 0.06);
  kgLabel.attr('opacity', n => connected.has(n.id) ? 1 : 0.06);
  kgLink.attr('opacity', l => {
    const src = typeof l.source === 'object' ? l.source.id : l.source;
    const tgt = typeof l.target === 'object' ? l.target.id : l.target;
    return (src === nodeId || tgt === nodeId) ? 1 : 0.02;
  });

  // Add pulse rings + arrow label
  kgG.selectAll('.kg-pulse-ring, .kg-arrow-group').remove();

  // Pulse rings
  for (let i = 0; i < 3; i++) {
    kgG.append('circle')
      .attr('class', 'kg-pulse-ring')
      .attr('cx', target.x)
      .attr('cy', target.y)
      .attr('r', 0)
      .style('animation-delay', `${i * 0.5}s`);
  }

  // Arrow label above node
  const arrowG = kgG.append('g').attr('class', 'kg-arrow-group');
  arrowG.append('text')
    .attr('class', 'kg-arrow-label')
    .attr('x', target.x)
    .attr('y', target.y - 45)
    .text('▼ ' + target.name);

  // Remove pulse after 5s
  setTimeout(() => {
    kgG.selectAll('.kg-pulse-ring, .kg-arrow-group').remove();
  }, 5000);
}

// Init — wait for DOM
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', kgLoadGraph);
} else {
  kgLoadGraph();
}

// Auto-refresh
setInterval(async () => {
  try {
    const resp = await fetch('/api/kg/graph');
    const data = await resp.json();
    if (data.entities.length !== kgAllData.entities.length || data.relations.length !== kgAllData.relations.length) {
      kgAllData = data;
      const sel = document.getElementById('kgIndustryFilter');
      const cur = sel.value;
      const industries = [...new Set(data.entities.filter(e => e.entity_type === 'industry').map(e => e.name))].sort();
      sel.innerHTML = '<option value="all">All Industries</option>';
      industries.forEach(ind => {
        const opt = document.createElement('option');
        opt.value = ind;
        opt.textContent = ind.charAt(0).toUpperCase() + ind.slice(1);
        sel.appendChild(opt);
      });
      sel.value = cur;
      kgApplyFilters();
    }
  } catch {}
}, 45000);
