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
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    background-color: #f8f9fa;
    color: #2c3e50;
  }

  .container {
    max-width: 1400px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    color: #2c3e50;
    margin-bottom: 2rem;
    font-size: 2.5rem;
    font-weight: 600;
    position: relative;
    padding-bottom: 0.5rem;
  }

  h1::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    width: 60px;
    height: 4px;
    background: #3498db;
    border-radius: 2px;
  }

  table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    margin-top: 1.5rem;
    background: white;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    border-radius: 8px;
    overflow: hidden;
  }

  th {
    background: #f8f9fa;
    font-weight: 600;
    text-align: left;
    position: sticky;
    top: 0;
    padding: 1rem;
    color: #2c3e50;
    border-bottom: 2px solid #e9ecef;
  }

  td {
    padding: 1rem;
    border-bottom: 1px solid #e9ecef;
    vertical-align: top;
  }

  tr:last-child td {
    border-bottom: none;
  }

  tr:hover {
    background-color: #f8f9fa;
  }

  pre {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
    max-height: 200px;
    overflow-y: auto;
    background: #f8f9fa;
    padding: 0.75rem;
    border-radius: 6px;
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Mono', 'Droid Sans Mono', 'Source Code Pro', monospace;
    font-size: 0.9em;
    line-height: 1.4;
  }

  .timestamp {
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Mono', 'Droid Sans Mono', 'Source Code Pro', monospace;
    color: #6c757d;
    white-space: nowrap;
  }

  .error {
    color: #dc3545;
    padding: 1rem;
    background: #f8d7da;
    border: 1px solid #f5c6cb;
    border-radius: 6px;
    margin-bottom: 1.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .empty-state {
    text-align: center;
    padding: 3rem;
    color: #6c757d;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  }

  .method {
    font-weight: 600;
    color: #3498db;
    white-space: nowrap;
    padding: 0.25rem 0.5rem;
    background: #e3f2fd;
    border-radius: 4px;
    display: inline-block;
  }

  .url {
    color: #2c3e50;
    word-break: break-all;
    font-family: 'SF Mono', 'Monaco', 'Inconsolata', 'Fira Mono', 'Droid Sans Mono', 'Source Code Pro', monospace;
  }

  .sub-table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    margin-top: 0.5rem;
    background: white;
    border-radius: 6px;
    overflow: hidden;
  }

  .sub-table th {
    background: #f8f9fa;
    font-weight: 500;
    font-size: 0.85em;
    padding: 0.5rem 0.75rem;
    color: #6c757d;
    border-bottom: 1px solid #e9ecef;
  }

  .sub-table td {
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid #e9ecef;
    font-size: 0.85em;
  }

  .sub-table tr:last-child td {
    border-bottom: none;
  }

  .sub-table tr:hover {
    background-color: #f8f9fa;
  }

  /* Scrollbar styling */
  pre::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  pre::-webkit-scrollbar-track {
    background: #f1f1f1;
    border-radius: 4px;
  }

  pre::-webkit-scrollbar-thumb {
    background: #c1c1c1;
    border-radius: 4px;
  }

  pre::-webkit-scrollbar-thumb:hover {
    background: #a8a8a8;
  }
</style>

<div class="container">
  <h1>Request Logger</h1>
  
  {#if error}
    <div class="error">
      <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M10 0C4.48 0 0 4.48 0 10C0 15.52 4.48 20 10 20C15.52 20 20 15.52 20 10C20 4.48 15.52 0 10 0ZM11 15H9V13H11V15ZM11 11H9V5H11V11Z" fill="currentColor"/>
      </svg>
      {error}
    </div>
  {/if}
  
  {#if requests.length === 0 && !error}
    <div class="empty-state">
      <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" style="margin-bottom: 1rem;">
        <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM12 20C7.59 20 4 16.41 4 12C4 7.59 7.59 4 12 4C16.41 4 20 7.59 20 12C20 16.41 16.41 20 12 20Z" fill="#6c757d"/>
        <path d="M13 7H11V13H17V11H13V7Z" fill="#6c757d"/>
      </svg>
      <p>No requests received yet.</p>
      <p style="color: #adb5bd; font-size: 0.9em;">Send a POST request to http://localhost:3000/requests to see it here.</p>
    </div>
  {:else}
    <table>
      <thead>
        <tr>
          <th>Timestamp</th>
          <th>Method</th>
          <th>URL</th>
          <th>Headers</th>
          <th>Body</th>
        </tr>
      </thead>
      <tbody>
        {#each requests as req}
          <tr>
            <td class="timestamp">{req.timestamp}</td>
            <td><span class="method">{req.request.method}</span></td>
            <td class="url">{req.request.url}</td>
            <td class="headers">
              {#if Object.keys(req.request.headers).length > 0}
                <table class="sub-table">
                  <thead>
                    <tr>
                      <th>Name</th>
                      <th>Value</th>
                    </tr>
                  </thead>
                  <tbody>
                    {#each Object.entries(req.request.headers) as [name, value]}
                      <tr>
                        <td>{name}</td>
                        <td>{value}</td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              {/if}
            </td>
            <td class="body">
              {#if req.request.body}
                <pre>{JSON.stringify(req.request.body, null, 2)}</pre>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

