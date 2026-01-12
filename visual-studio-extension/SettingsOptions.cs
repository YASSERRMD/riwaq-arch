using System;
using System.ComponentModel;
using System.Runtime.InteropServices;

namespace RiwaqArch
{
    [ComVisible(true)]
    [Guid("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")]
    public class SettingsOptions : DialogPage
    {
        private static SettingsOptions _instance;
        public static SettingsOptions Instance => _instance ??= new SettingsOptions();

        [Category("Server")]
        [DisplayName("Server URL")]
        [Description("The URL where the Riwaq server is running")]
        public string ServerUrl { get; set; } = "http://127.0.0.1:9527";

        [Category("Server")]
        [DisplayName("Auto-start Server")]
        [Description("Automatically start the server when Visual Studio opens")]
        public bool AutoStartServer { get; set; } = true;

        [Category("LLM")]
        [DisplayName("API Key")]
        [Description("Your LLM provider API key (OpenRouter, OpenAI, etc.)")]
        [PasswordPropertyText(true)]
        public string LLMApiKey { get; set; } = "";

        [Category("LLM")]
        [DisplayName("API Endpoint")]
        [Description("The LLM API endpoint URL")]
        public string LLMEndpoint { get; set; } = "https://openrouter.ai/api/v1";

        [Category("LLM")]
        [DisplayName("Model")]
        [Description("The LLM model to use")]
        public string LLMModel { get; set; } = "glm-4";

        [Category("Analysis")]
        [DisplayName("Excluded Directories")]
        [Description("Comma-separated list of directories to exclude from analysis")]
        public string ExcludedDirs { get; set; } = "node_modules,dist,build,target,.git,bin,obj";
    }
}
