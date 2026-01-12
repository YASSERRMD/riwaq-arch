using System;
using System.Net.Http;
using System.Threading.Tasks;
using Microsoft.VisualStudio.Shell;
using Newtonsoft.Json;

namespace RiwaqArch
{
    public class RiwaqClient
    {
        private readonly HttpClient _httpClient;
        private readonly string _baseUrl;

        public RiwaqClient(string baseUrl = "http://127.0.0.1:9527")
        {
            _httpClient = new HttpClient
            {
                Timeout = TimeSpan.FromSeconds(60)
            };
            _baseUrl = baseUrl;
        }

        public async Task<bool> IsServerAvailableAsync()
        {
            try
            {
                var response = await _httpClient.GetAsync($"{_baseUrl}/health");
                return response.IsSuccessStatusCode;
            }
            catch
            {
                return false;
            }
        }

        public async Task<AnalyzeResponse> AnalyzeProjectAsync(string projectPath, string[] excludedDirs)
        {
            var request = new AnalyzeRequest
            {
                projectPath = projectPath,
                excludedDirs = excludedDirs ?? System.Array.Empty<string>()
            };

            var content = new StringContent(JsonConvert.SerializeObject(request));
            content.Headers.ContentType = new System.Net.Http.Headers.MediaTypeHeaderValue("application/json");

            var response = await _httpClient.PostAsync($"{_baseUrl}/api/analyze", content);
            var responseString = await response.Content.ReadAsStringAsync();

            return JsonConvert.DeserializeObject<AnalyzeResponse>(responseString);
        }

        public async Task<AnalysisData> GetAnalysisAsync()
        {
            var response = await _httpClient.GetAsync($"{_baseUrl}/api/analysis");
            var responseString = await response.Content.ReadAsStringAsync();

            return JsonConvert.DeserializeObject<AnalysisData>(responseString);
        }

        public async Task<DiagramData> GetArchitectureDiagramAsync()
        {
            var response = await _httpClient.GetAsync($"{_baseUrl}/api/diagram");
            var responseString = await response.Content.ReadAsStringAsync();

            return JsonConvert.DeserializeObject<DiagramData>(responseString);
        }
    }

    // Data models
    public class AnalyzeRequest
    {
        public string projectPath { get; set; }
        public string[] excludedDirs { get; set; }
    }

    public class AnalyzeResponse
    {
        public bool success { get; set; }
        public string message { get; set; }
        public string analysisId { get; set; }
    }

    public class AnalysisData
    {
        public ModuleInfo[] modules { get; set; }
        public ServiceInfo[] services { get; set; }
        public AnalysisStats stats { get; set; }
    }

    public class ModuleInfo
    {
        public string name { get; set; }
        public string path { get; set; }
        public string type { get; set; }
        public string health { get; set; }
        public ModuleMetrics metrics { get; set; }
        public string[] dependencies { get; set; }
        public string[] dependents { get; set; }
        public string[] publicApi { get; set; }
        public string description { get; set; }
    }

    public class ModuleMetrics
    {
        public int linesOfCode { get; set; }
        public int functionCount { get; set; }
        public int typeCount { get; set; }
        public int coupling { get; set; }
        public double cohesion { get; set; }
    }

    public class ServiceInfo
    {
        public string name { get; set; }
        public string path { get; set; }
        public string[] entryPoints { get; set; }
    }

    public class AnalysisStats
    {
        public int totalModules { get; set; }
        public int totalServices { get; set; }
        public int totalLinesOfCode { get; set; }
        public double averageCoupling { get; set; }
        public double averageCohesion { get; set; }
    }

    public class DiagramData
    {
        public string mermaid { get; set; }
        public DiagramStats stats { get; set; }
        public DiagramModule[] modules { get; set; }
    }

    public class DiagramStats
    {
        public int totalModules { get; set; }
        public int totalConnections { get; set; }
        public int maxDepth { get; set; }
    }

    public class DiagramModule
    {
        public string id { get; set; }
        public string name { get; set; }
        public string type { get; set; }
        public int level { get; set; }
        public string[] dependencies { get; set; }
        public string[] dependents { get; set; }
    }
}
