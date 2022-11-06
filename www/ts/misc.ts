/**
 * @author    Yannick Deubel (https://github.com/yandeu)
 * @copyright Copyright (c) 2022 Yannick Deubel
 * @license   {@link https://github.com/yandeu/docker-swarm-visualizer/blob/main/LICENSE LICENSE}
 */

export const toPercent = value => {
  return (value * 100).toFixed(2) + '%'
}

export const toMb = value => {
  return Math.round(value / 1000 / 1000)
}

export const toMiB = value => {
  return Math.round(value / 1024 / 1024)
}

export const toGb = value => {
  return (value / 1000 / 1000 / 1000).toFixed(3)
}

export const toGiB = value => {
  return (value / 1024 / 1024 / 1024).toFixed(3)
}

export const ipToId = id => {
  return id.replace(/\./gm, '-')
}

export const addOrangeCircle = ip => {
  const id = ipToId(ip)

  const nodeHTML = document.getElementById(id)
  if (!nodeHTML) return

  const circle = nodeHTML.querySelector('.circle')
  if (!circle) return

  circle.classList.remove('blink')
  circle.classList.replace('yellow', 'orange')
}

export const calculateCPUUsage = stats => {
  // does not work on windows
  // https://github.com/moby/moby/blob/eb131c5383db8cac633919f82abad86c99bffbe5/cli/command/container/stats_helpers.go#L175-L188
  // https://stackoverflow.com/questions/35692667/in-docker-cpu-usage-calculation-what-are-totalusage-systemusage-percpuusage-a
  // https://docs.docker.com/config/containers/runmetrics/

  let cpuPercent = 0.0

  try {
    const cpuDelta = stats.cpu_stats.cpu_usage.total_usage - stats.precpu_stats.cpu_usage.total_usage

    const systemDelta = stats.cpu_stats.system_cpu_usage - stats.precpu_stats.system_cpu_usage

    if (systemDelta > 0.0 && cpuDelta > 0.0) cpuPercent = (cpuDelta / systemDelta) * stats.cpu_stats.online_cpus * 100.0

    return cpuPercent.toFixed(0) + '%'
  } catch (error: any) {
    console.log(error.message)
    return cpuPercent.toFixed(0) + '%'
  }
}
