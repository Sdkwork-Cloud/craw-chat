// SDKWork IM Audit Report — chart logic
(function () {
  var style = getComputedStyle(document.documentElement);
  var accent = style.getPropertyValue('--accent').trim();
  var accent2 = style.getPropertyValue('--accent2').trim();
  var ink = style.getPropertyValue('--ink').trim();
  var muted = style.getPropertyValue('--muted').trim();
  var rule = style.getPropertyValue('--rule').trim();
  var bg2 = style.getPropertyValue('--bg2').trim();
  var crit = style.getPropertyValue('--crit').trim();
  var high = style.getPropertyValue('--high').trim();
  var med = style.getPropertyValue('--med').trim();
  var low = style.getPropertyValue('--low').trim();

  var charts = [];

  function make(id, opt) {
    var el = document.getElementById(id);
    if (!el || typeof echarts === 'undefined') return null;
    var c = echarts.init(el, null, { renderer: 'svg' });
    c.setOption(opt);
    charts.push(c);
    return c;
  }

  // --- Chart: Issue domain distribution (bar) ---
  make('chart-domain', {
    animation: false,
    tooltip: { trigger: 'axis', axisPointer: { type: 'shadow' }, appendToBody: true },
    grid: { left: 10, right: 24, top: 30, bottom: 10, containLabel: true },
    xAxis: {
      type: 'category',
      data: ['安全', '可靠性/HA', '供应链', '可观测性', '前端/SDK', '质量门禁', '后端健壮性', '文档一致性'],
      axisLine: { lineStyle: { color: rule } },
      axisTick: { show: false },
      axisLabel: { color: muted, fontSize: 11, interval: 0, rotate: 0 }
    },
    yAxis: {
      type: 'value',
      name: '发现数量',
      nameTextStyle: { color: muted, fontSize: 11 },
      axisLine: { show: false },
      splitLine: { lineStyle: { color: rule } },
      axisLabel: { color: muted, fontSize: 11 }
    },
    series: [{
      type: 'bar',
      data: [
        { value: 11, itemStyle: { color: crit } },
        { value: 5,  itemStyle: { color: crit } },
        { value: 5,  itemStyle: { color: crit } },
        { value: 2,  itemStyle: { color: high } },
        { value: 10, itemStyle: { color: high } },
        { value: 3,  itemStyle: { color: med } },
        { value: 3,  itemStyle: { color: med } },
        { value: 1,  itemStyle: { color: low } }
      ],
      barWidth: '52%',
      label: { show: true, position: 'top', color: ink, fontWeight: 600, fontSize: 12 }
    }]
  });

  // --- Chart: Commercialization readiness radar ---
  make('chart-readiness', {
    animation: false,
    tooltip: { appendToBody: true },
    radar: {
      indicator: [
        { name: '安全防护', max: 100 },
        { name: '可靠性/高可用', max: 100 },
        { name: '供应链安全', max: 100 },
        { name: '可观测性', max: 100 },
        { name: '前端/客户端', max: 100 },
        { name: '运维/灾备', max: 100 },
        { name: '质量门禁', max: 100 },
        { name: '文档/合规一致性', max: 100 }
      ],
      center: ['50%', '54%'],
      radius: '66%',
      axisName: { color: ink, fontSize: 12 },
      splitLine: { lineStyle: { color: rule } },
      splitArea: { areaStyle: { color: [bg2, 'transparent'] } },
      axisLine: { lineStyle: { color: rule } }
    },
    series: [{
      type: 'radar',
      data: [{
        value: [55, 45, 30, 50, 55, 35, 60, 50],
        name: '当前就绪度',
        areaStyle: { color: accent, opacity: 0.22 },
        lineStyle: { color: accent, width: 2 },
        itemStyle: { color: accent }
      }]
    }]
  });

  window.addEventListener('resize', function () {
    charts.forEach(function (c) { c.resize(); });
  });
})();
