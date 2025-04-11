<script>
  import axios from 'axios';
  
  let requests = [];
  let error = null;
  const BACKEND_URL = '/api';

  async function fetchRequests() {
    try {
      error = null;
      const response = await axios.get(`${BACKEND_URL}/requests`);
      requests = response.data;
    } catch (error) {
      console.error('Error fetching requests:', error);
      error = 'Failed to fetch requests. Make sure the backend is running.';
    }
  }

  // Fetch requests immediately and then every 2 seconds
  fetchRequests();
  setInterval(fetchRequests, 2000);
</script>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 1rem;
    background: white;
    box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  }

  th {
    background: #f5f5f5;
    font-weight: 600;
    text-align: left;
  }

  td, th {
    border: 1px solid #ddd;
    padding: 0.75rem;
    text-align: left;
  }

  tr:nth-child(even) {
    background: #f9f9f9;
  }

  pre {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
  }

  h1 {
    color: #333;
    margin-bottom: 1.5rem;
  }

  .timestamp {
    font-family: monospace;
    color: #666;
  }

  .error {
    color: #dc3545;
    padding: 1rem;
    background: #f8d7da;
    border: 1px solid #f5c6cb;
    border-radius: 4px;
    margin-bottom: 1rem;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: #666;
  }
</style>

<div class="container">
  <h1>Request Logger</h1>
  
  {#if error}
    <div class="error">
      {error}
    </div>
  {/if}
  
  {#if requests.length === 0 && !error}
    <div class="empty-state">
      No requests received yet. Send a POST request to http://localhost:3000 to see it here.
    </div>
  {:else}
    <table>
      <thead>
        <tr>
          <th>Timestamp</th>
          <th>Request Body</th>
        </tr>
      </thead>
      <tbody>
        {#each requests as req}
          <tr>
            <td class="timestamp">{req.timestamp}</td>
            <td><pre>{JSON.stringify(req.body, null, 2)}</pre></td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

