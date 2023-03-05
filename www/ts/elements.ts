/**
 * @author    Yannick Deubel (https://github.com/yandeu)
 * @copyright Copyright (c) 2022 Yannick Deubel
 * @license   {@link https://github.com/yandeu/docker-swarm-visualizer/blob/main/LICENSE LICENSE}
 */

import { ipToId, calculateCPUUsage, toMiB } from './misc.js'

// keep track of services
const services: string[] = []
// keep track of deleted containers
const deletedContainers: string[] = []

const node = node => {
  const { address, role, availability, state } = node

  const status = state === 'down' ? 'red' : 'yellow blink'

  const placeholders = `<li class="usage">...&nbsp;&nbsp;/&nbsp;&nbsp;...&nbsp;&nbsp;/&nbsp;&nbsp;...</li>
    <li class="usage_percent">...&nbsp;&nbsp;/&nbsp;&nbsp;...&nbsp;&nbsp;/&nbsp;&nbsp;...</li>`

  return `
    <div class="node ${role} ${state}" id="${ipToId(address)}">
      <div class="node-info">
        <ul>
          <li>
            <div style="
              display: flex;
              justify-content: center;
              position: relative;
              left: -9px;
              margin-bottom: 6px;"
              >
                <p class="circle ${status} ${role}"></p>
                <b>${address}&nbsp;</b>
                <!-- ${role === 'manager' ? '<b class="upload-action">⇪</b>' : ''} -->
            </div>
          </li>
          <li class="os">-</li>
          <li style="margin-bottom: 8px;">${role} / ${availability} / ${state}</li>
          ${state !== 'down' ? placeholders : ''}
        </ul>
      </div>
      <div class="node-containers"></div>             
    </div>`
}

const completeNode = (id, ip /* internal IP*/, containers) => {
  const nodeHTML = document.getElementById(id)
  if (nodeHTML) {
    const child = nodeHTML.lastElementChild // node-container

    let currentIds = child ? Array.from(child.children).map(child => child.id) : []

    containers.forEach(container => {
      const containerId = container.id

      // check nodeAddress as fetched
      if (currentIds) {
        const index = currentIds.indexOf(containerId)
        if (index > -1) currentIds.splice(index, 1)
      }

      const existing = document.getElementById(containerId)

      // replace
      if (existing) {
        existing.replaceWith(container)
      }
      // add new
      else {
        if (child) child.appendChild(container)
      }
    })

    // remove container that do not exist anymore

    if (currentIds)
      currentIds.forEach(id => {
        const el = document.getElementById(id)
        if (el) el.remove()
      })
  }

  const circle = nodeHTML ? nodeHTML.querySelector('.circle') : null
  if (circle) {
    circle.classList.remove('blink')
    circle.classList.replace('yellow', 'green')
  }

  // add listeners to action (for now only remove action)
  const actions: any = nodeHTML ? nodeHTML.querySelectorAll('.action') : []
  actions.forEach(action => {
    action.addEventListener('click', () => {
      const id = action.parentElement.getAttribute('id')
      console.log('remove container', id, 'on', ip)

      fetch(`/api/dev/agent/${ip}/containers/${id}`, { method: 'DELETE' })
        .then(() => {
          deletedContainers.push(id)
          action.parentElement.classList.add('deleting')
          setTimeout(() => {
            action.parentElement.remove()
          }, 650)
          console.log('Successfully removed!')
        })
        .catch(() => {
          console.warn('Could not remove container.')
        })
    })
  })
}

const container = (container, _MemTotal) => {
  // TODO:(yandeu) re-implement cpu and mem usage

  // https://docs.docker.com/engine/api/v1.42/#tag/Container/operation/ContainerStats
  const memory_stats = container.stats.memory_stats
  const used_memory = memory_stats.usage - (memory_stats?.stats?.cache || 0)
  const available_memory = memory_stats.limit
  const memoryUsagePercent = (used_memory / available_memory) * 100.0

  const cpuUsage = calculateCPUUsage(container.stats)

  const { image, names, labels, state, status, id } = container

  if (deletedContainers.indexOf(id) > 0) return 'DELETED'

  // add colors to services
  const colors = ['blue', 'yellow', 'red', 'green', 'orange', 'violet']
  let service = labels['com.docker.swarm.service.name'] ?? ''
  if (service && !services.includes(service)) services.push(service)
  const color = service && colors[services.indexOf(service) % colors.length]

  const name =
    labels['com.docker.swarm.service.name'] || `${image} / ${names.map(n => n.replace(/^\//gm, '')).join(', ')}`
  const action = state !== 'running' ? `<button class="action action_remove">🗑</button>` : ''

  const html = `
    <div class="container ${state.toLowerCase()} ${service && color}" id="${id}">
      ${action}
      <ul>
        <li><b style="font-size: 14px;"class="is-${service && color}">${name}</b></li>
        <li>${state}</li>
        <li>${status}</li>
        <li>MEM ${toMiB(used_memory)}MiB</li>
        <li>MEM ${memoryUsagePercent.toFixed(2)}%</li>
        <li>CPU ${cpuUsage}</li>
      </ul>
    </div>
    `.trim()

  const template = document.createElement('template')
  template.innerHTML = html.trim()

  return template.content.firstChild
}

export const elements = {
  node,
  container,
  complete: {
    node: completeNode
  }
}
