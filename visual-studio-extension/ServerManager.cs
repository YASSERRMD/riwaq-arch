using System;
using System.IO;
using System.Diagnostics;
using System.Threading.Tasks;
using Microsoft.VisualStudio.Shell;
using Microsoft.VisualStudio.Shell.Interop;

namespace RiwaqArch
{
    public class ServerManager : IDisposable
    {
        private Process _serverProcess;
        private readonly string _binaryPath;
        private readonly string _serverUrl = "http://127.0.0.1:9527";

        public bool IsRunning => _serverProcess != null && !_serverProcess.HasExited;

        public ServerManager()
        {
            var assemblyLocation = Assembly.GetExecutingAssembly().Location;
            var assemblyDirectory = Path.GetDirectoryName(assemblyLocation);
            _binaryPath = Path.Combine(assemblyDirectory, "bin", "riwaq.exe");

            if (!File.Exists(_binaryPath))
            {
                // Try alternative locations
                var alternativePaths = new[]
                {
                    Path.Combine(assemblyDirectory, "riwaq.exe"),
                    Path.Combine(assemblyDirectory, "..", "..", "..", "..", "vscode-extension", "bin", "riwaq.exe")
                };

                foreach (var path in alternativePaths)
                {
                    if (File.Exists(path))
                    {
                        _binaryPath = Path.GetFullPath(path);
                        break;
                    }
                }
            }
        }

        public async Task InitializeAsync()
        {
            await Task.CompletedTask; // Placeholder for async initialization
        }

        public async Task<bool> StartServerAsync()
        {
            if (IsRunning) return true;

            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            try
            {
                if (!File.Exists(_binaryPath))
                {
                    VsShellUtilities.ShowMessageBox(
                        this as IAsyncServiceProvider,
                        $"Riwaq binary not found at: {_binaryPath}",
                        "Server Start Failed",
                        OLEMSGICON.OLEMSGICON_CRITICAL,
                        OLEMSGBUTTON.OLEMSGBUTTON_OK,
                        OLEMSGDEFBUTTON.OLEMSGDEFBUTTON_FIRST);
                    return false;
                }

                var startInfo = new ProcessStartInfo
                {
                    FileName = _binaryPath,
                    Arguments = "serve --host 127.0.0.1 --port 9527",
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    RedirectStandardError = true,
                    CreateNoWindow = true
                };

                // Set environment variables for LLM configuration
                var settings = SettingsOptions.Instance;
                if (!string.IsNullOrEmpty(settings.LLMApiKey))
                {
                    startInfo.Environment["RIWAQ_LLM_API_KEY"] = settings.LLMApiKey;
                }
                if (!string.IsNullOrEmpty(settings.LLMEndpoint))
                {
                    startInfo.Environment["RIWAQ_LLM_ENDPOINT"] = settings.LLMEndpoint;
                }
                if (!string.IsNullOrEmpty(settings.LLMModel))
                {
                    startInfo.Environment["RIWAQ_LLM_MODEL"] = settings.LLMModel;
                }

                _serverProcess = Process.Start(startInfo);
                return await WaitForServerReadyAsync();
            }
            catch (Exception ex)
            {
                VsShellUtilities.ShowMessageBox(
                    this as IAsyncServiceProvider,
                    $"Failed to start server: {ex.Message}",
                    "Server Start Error",
                    OLEMSGICON.OLEMSGICON_CRITICAL,
                    OLEMSGBUTTON.OLEMSGBUTTON_OK,
                    OLEMSGDEFBUTTON.OLEMSGDEFBUTTON_FIRST);
                return false;
            }
        }

        public async Task<bool> StopServerAsync()
        {
            if (!IsRunning) return true;

            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            try
            {
                _serverProcess?.Kill();
                _serverProcess?.WaitForExit(5000);
                _serverProcess = null;
                return true;
            }
            catch
            {
                return false;
            }
        }

        public async Task<bool> RestartServerAsync()
        {
            await StopServerAsync();
            await Task.Delay(1000);
            return await StartServerAsync();
        }

        private async Task<bool> WaitForServerReadyAsync()
        {
            // Simple health check with retry
            var client = new System.Net.Http.HttpClient();
            client.Timeout = TimeSpan.FromSeconds(30);

            for (int i = 0; i < 60; i++)
            {
                try
                {
                    var response = await client.GetAsync($"{_serverUrl}/health");
                    if (response.IsSuccessStatusCode)
                    {
                        return true;
                    }
                }
                catch
                {
                    // Server not ready yet
                }
                await Task.Delay(500);
            }

            return false;
        }

        public void Dispose()
        {
            _serverProcess?.Kill();
            _serverProcess?.Dispose();
        }
    }
}
