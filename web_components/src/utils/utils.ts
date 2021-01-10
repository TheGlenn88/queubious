import { QueueDataInterface } from '../components/waiting-room/queue-data.interface';

export function getQueueData(url): Promise<QueueDataInterface> {
  return fetch(url)
    .then(response => response.json());
}
