using System;
using System.IO;
using System.Windows.Forms;
using Microsoft.VisualStudio.Shell;

namespace RiwaqArch.ToolWindows
{
    [Guid(GuidList.guidArchitectureToolWindowString)]
    public class ArchitectureToolWindow : ToolWindowPane
    {
        private RiwaqClient _client;
        private TabControl _tabControl;
        private ModulesPanel _modulesPanel;
        private AskPanel _askPanel;
        private DiagramPanel _diagramPanel;
        private InsightsPanel _insightsPanel;

        public ArchitectureToolWindow() : base(null)
        {
            this.Caption = "Riwaq Architecture";

            _client = new RiwaqClient(SettingsOptions.Instance.ServerUrl);

            // Create main tab control
            _tabControl = new TabControl
            {
                Dock = DockStyle.Fill
            };

            // Create panels
            _modulesPanel = new ModulesPanel();
            _askPanel = new AskPanel();
            _diagramPanel = new DiagramPanel();
            _insightsPanel = new InsightsPanel();

            // Add tabs
            var tabPage1 = new TabPage("Modules");
            tabPage1.Controls.Add(_modulesPanel);
            _tabControl.TabPages.Add(tabPage1);

            var tabPage2 = new TabPage("Ask AI");
            tabPage2.Controls.Add(_askPanel);
            _tabControl.TabPages.Add(tabPage2);

            var tabPage3 = new TabPage("Diagram");
            tabPage3.Controls.Add(_diagramPanel);
            _tabControl.TabPages.Add(tabPage3);

            var tabPage4 = new TabPage("Insights");
            tabPage4.Controls.Add(_insightsPanel);
            _tabControl.TabPages.Add(tabPage4);
        }

        protected override void Initialize()
        {
            base.Initialize();
            this.Content = _tabControl;

            // Load initial data
            RefreshDataAsync();
        }

        public async System.Threading.Tasks.Task RefreshDataAsync()
        {
            if (await _client.IsServerAvailableAsync())
            {
                await _modulesPanel.LoadDataAsync(_client);
                await _insightsPanel.LoadDataAsync(_client);
            }
        }
    }

    // Simplified panel base class
    public class ModulesPanel : UserControl
    {
        private ListView _listView;

        public ModulesPanel()
        {
            _listView = new ListView
            {
                Dock = DockStyle.Fill,
                View = View.Details
            };
            _listView.Columns.Add("Module", 200);
            _listView.Columns.Add("Health", 80);
            _listView.Columns.Add("LoC", 80);
            _listView.Columns.Add("Coupling", 80);

            this.Controls.Add(_listView);
        }

        public async System.Threading.Tasks.Task LoadDataAsync(RiwaqClient client)
        {
            try
            {
                var data = await client.GetAnalysisAsync();
                _listView.Items.Clear();

                foreach (var module in data.modules)
                {
                    var item = new ListViewItem(module.name);
                    item.SubItems.Add(GetHealthIcon(module.health));
                    item.SubItems.Add(module.metrics.linesOfCode.ToString());
                    item.SubItems.Add(module.metrics.coupling.ToString());
                    _listView.Items.Add(item);
                }
            }
            catch (Exception ex)
            {
                System.Windows.Forms.MessageBox.Show($"Error loading modules: {ex.Message}");
            }
        }

        private string GetHealthIcon(string health)
        {
            return health switch
            {
                "good" => "🟢",
                "warning" => "🟡",
                "poor" => "🔴",
                _ => "⚪"
            };
        }
    }

    public class AskPanel : UserControl
    {
        private TextBox _questionTextBox;
        private Button _askButton;
        private TextBox _answerTextBox;

        public AskPanel()
        {
            var inputPanel = new Panel
            {
                Dock = DockStyle.Top,
                Height = 60
            };

            _questionTextBox = new TextBox
            {
                Location = new System.Drawing.Point(10, 10),
                Width = 400
            };

            _askButton = new Button
            {
                Text = "Ask",
                Location = new System.Drawing.Point(420, 8),
                Width = 80
            };
            _askButton.Click += async (sender, e) => await AskQuestionAsync();

            inputPanel.Controls.Add(_questionTextBox);
            inputPanel.Controls.Add(_askButton);

            _answerTextBox = new TextBox
            {
                Dock = DockStyle.Fill,
                Multiline = true,
                ReadOnly = true,
                ScrollBars = ScrollBars.Vertical
            };

            this.Controls.Add(_answerTextBox);
            this.Controls.Add(inputPanel);
        }

        private async System.Threading.Tasks.Task AskQuestionAsync()
        {
            var question = _questionTextBox.Text;
            if (string.IsNullOrWhiteSpace(question)) return;

            _answerTextBox.Text = "Thinking...";

            // Placeholder for actual implementation
            await System.Threading.Tasks.Task.Delay(1000);
            _answerTextBox.Text = $"Answer to: {question}\n\nStreaming responses will be implemented here.";
        }
    }

    public class DiagramPanel : UserControl
    {
        private WebBrowser _browser;

        public DiagramPanel()
        {
            _browser = new WebBrowser
            {
                Dock = DockStyle.Fill
            };

            this.Controls.Add(_browser);
        }

        public async System.Threading.Tasks.Task LoadDataAsync(RiwaqClient client)
        {
            try
            {
                var data = await client.GetArchitectureDiagramAsync();
                var html = GenerateDiagramHtml(data.mermaid);
                _browser.DocumentText = html;
            }
            catch (Exception ex)
            {
                _browser.DocumentText = $"<html><body>Error: {ex.Message}</body></html>";
            }
        }

        private string GenerateDiagramHtml(string mermaid)
        {
            return $@"
<!DOCTYPE html>
<html>
<head>
    <script type='module'>
        import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
        mermaid.initialize({{ startOnLoad: true }});
    </script>
    <style>
        body {{ font-family: Arial, sans-serif; padding: 20px; }}
        pre {{ background-color: #f5f5f5; padding: 10px; border-radius: 5px; }}
    </style>
</head>
<body>
    <h3>Architecture Diagram</h3>
    <pre class='mermaid'>
{mermaid}
    </pre>
</body>
</html>";
        }
    }

    public class InsightsPanel : UserControl
    {
        private ListView _statsListView;

        public InsightsPanel()
        {
            _statsListView = new ListView
            {
                Dock = DockStyle.Fill,
                View = View.Details
            };
            _statsListView.Columns.Add("Metric", 200);
            _statsListView.Columns.Add("Value", 200);

            this.Controls.Add(_statsListView);
        }

        public async System.Threading.Tasks.Task LoadDataAsync(RiwaqClient client)
        {
            try
            {
                var data = await client.GetAnalysisAsync();
                var stats = data.stats;

                _statsListView.Items.Clear();
                _statsListView.Items.Add(new ListViewItem(new[] { "Total Modules", stats.totalModules.ToString() }));
                _statsListView.Items.Add(new ListViewItem(new[] { "Total Services", stats.totalServices.ToString() }));
                _statsListView.Items.Add(new ListViewItem(new[] { "Total Lines of Code", stats.totalLinesOfCode.ToString() }));
                _statsListView.Items.Add(new ListViewItem(new[] { "Average Coupling", stats.averageCoupling.ToString("F2") }));
                _statsListView.Items.Add(new ListViewItem(new[] { "Average Cohesion", stats.averageCohesion.ToString("F2") }));
            }
            catch (Exception ex)
            {
                System.Windows.Forms.MessageBox.Show($"Error loading insights: {ex.Message}");
            }
        }
    }
}
