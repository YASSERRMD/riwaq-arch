using System;
using System.Design;
using Microsoft.VisualStudio.Shell;
using Microsoft.VisualStudio.Shell.Interop;

namespace RiwaqArch
{
    internal static class GenerateDocsCommand
    {
        public static async System.Threading.Tasks.Task InitializeAsync(RiwaqArchPackage package)
        {
            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            var commandId = new CommandID(GuidList.guidRiwaqArchCmdSet, (int)PkgCmdIDList.cmdidGenerateDocs);
            var menuItem = new OleMenuCommand((sender, e) => ExecuteAsync(package).Wait(), commandId);
            menuItem.BeforeQueryStatus += (sender, e) => MenuItem_BeforeQueryStatus(sender, e);

            var mcs = await package.GetServiceAsync(typeof(IMenuCommandService)) as OleMenuCommandService;
            mcs?.AddCommand(menuItem);
        }

        private static void MenuItem_BeforeQueryStatus(object sender, EventArgs e)
        {
            var button = (OleMenuCommand)sender;
            button.Enabled = true;
            button.Visible = true;
        }

        private static async System.Threading.Tasks.Task ExecuteAsync(RiwaqArchPackage package)
        {
            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            try
            {
                var dte = await package.GetServiceAsync(typeof(EnvDTE.DTE)) as EnvDTE.DTE;
                if (dte?.Solution is not null && dte.Solution.IsOpen)
                {
                    var solutionPath = dte.Solution.FullName;
                    var solutionDir = System.IO.Path.GetDirectoryName(solutionPath);
                    var outputDir = System.IO.Path.Combine(solutionDir, "riwaq-docs");

                    var client = new RiwaqClient();
                    
                    // Trigger async generation
                    var result = await client.GenerateDocsAsync(solutionDir, outputDir);

                    if (result.success)
                    {
                        VsShellUtilities.ShowMessageBox(
                            package,
                            $"Documentation generation completed!\nFiles: {result.fileCount}\nLocation: {result.outputDir}",
                            "Riwaq Arch",
                            OLEMSGICON.OLEMSGICON_INFO,
                            OLEMSGBUTTON.OLEMSGBUTTON_OK,
                            OLEMSGDEFBUTTON.OLEMSGDEFBUTTON_FIRST);
                    }
                    else
                    {
                        VsShellUtilities.ShowMessageBox(
                            package,
                            $"Generation failed.",
                            "Riwaq Arch",
                            OLEMSGICON.OLEMSGICON_CRITICAL,
                            OLEMSGBUTTON.OLEMSGBUTTON_OK,
                            OLEMSGDEFBUTTON.OLEMSGDEFBUTTON_FIRST);
                    }
                }
            }
            catch (Exception ex)
            {
                VsShellUtilities.ShowMessageBox(
                    package,
                    $"Error generating docs: {ex.Message}",
                    "Riwaq Arch",
                    OLEMSGICON.OLEMSGICON_CRITICAL,
                    OLEMSGBUTTON.OLEMSGBUTTON_OK,
                    OLEMSGDEFBUTTON.OLEMSGDEFBUTTON_FIRST);
            }
        }
    }
}
