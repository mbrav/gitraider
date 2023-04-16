local dap = require("dap")

-- Build binary 
-- vim.fn.jobstart("cargo build") 

dap.adapters.codelldb = {
  type = "server",
  port = "${port}",
  executable = {
    command = "/usr/bin/codelldb",
    args = {"--port", "${port}"},
  }
}

dap.configurations.rust = {
  {
    name = "Rust debug",
    type = "codelldb",
    request = "launch",
    showDisassembly = "never",
    args =["--push"],
    program = function()
      return vim.fn.input("Path to executable: ", vim.fn.getcwd() .. "/target/debug/", "gitraider")
    end,
    cwd = "${workspaceFolder}",
    stopOnEntry = true,
  },
}
