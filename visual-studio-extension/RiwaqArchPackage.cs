using System;
using System.Reflection;
using System.Runtime.InteropServices;
using Microsoft.VisualStudio.Shell;

namespace RiwaqArch
{
    [PackageRegistration(UseManagedResourcesOnly = true, AllowsBackgroundLoading = true)]
    [InstalledProductRegistration("#110", "#112", "1.0.0", IconResourceID = 400)]
    [ProvideMenuResource("Menus.ctmenu", 1, 2)]
    [ProvideToolWindow(typeof(ArchitectureToolWindow), Style = VsDockStyle.Tabbed, Window = EnvDTE.Constants.vsWindowKindOutput)]
    [Guid(GuidList.guidRiwaqArchPkgString)]
    [ProvideOptionPage(typeof(SettingsOptions), "Riwaq Arch", "General", 0, 0, true)]
    public sealed class RiwaqArchPackage : AsyncPackage
    {
        public const string PackageGuidString = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx";

        public static readonly Guid GuidRiwaqArchCmdSet = new Guid("{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}");

        public static ServerManager ServerManager { get; private set; }

        public RiwaqArchPackage()
        {
            // Inside this method you can place any initialization code that does not require
            // Visual Studio to be fully initialized.
        }

        protected override async System.Threading.Tasks.Task InitializeAsync(System.Threading.CancellationToken cancellationToken, IProgress<ServiceProgressData> progress)
        {
            await JoinableTaskFactory.SwitchToMainThreadAsync(cancellationToken);

            ServerManager = new ServerManager();
            await ServerManager.InitializeAsync();

            // Initialize tool window
            await ShowToolWindowAsync(typeof(ArchitectureToolWindow), 0, createIfNeeded: true, cancellationToken);

            // Initialize commands
            await AnalyzeProjectCommand.InitializeAsync(this);
            await StartServerCommand.InitializeAsync(this);
            await StopServerCommand.InitializeAsync(this);
        }

        protected override void Dispose(bool disposing)
        {
            base.Dispose(disposing);
            ServerManager?.Dispose();
        }
    }
}
