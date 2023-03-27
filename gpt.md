# GPT

Ok _wise and salty one_, we're going to work through a problem together.

I'm tasked with creating a command line rust application named `loopy` that performs the following;

1. [x] takes command line arguments using the `clap` crate.

2. [x] includes logging at levels debug, info, warn, error where appropriate. logging will go to both stdout and a log file which has a configurable location via a command line flag or location in config.yaml

3. [ ] includes comments in every function compatible with `rustdoc`

4. [x] checks for a `config.yaml` configuration file the location of which is either provided with `--config` or defaults to `config.yaml` in the current directory. (I will provide the `config.yaml` for context on the structure and layout)

5. [x] reads and ensure the configuration file is valid yaml before continuing

6. [ ] from the configuration file section `dependencies.tools` checks the PATH for each binary to see if its already available using the name `dependencies.tools.name.bin`. If the tool is not available and a URL is listed, offer to download the tool into a folder called `bin` in the current directory. If downloaded, this is the location of the tool that will be used for future commands using it. If the tool is not available and no URL is listed, exit with an error asking the user to install the tool manually.

7. [ ] within the config file a section `dependencies.helm.repositories` lists all helm repositories that will be installed

8. [ ] within the config file a section `dependencies.helm.charts` lists all helm charts that will have two things done with them described in the next two steps.

9. [ ] gather the default values file using `helm show values` and write it to helm/charts/$chart_name/defaults.yaml`

10. [ ] Install the helm chart with default values from defaults.yaml OR if the file `values.yaml` exists in that same folder, use that file instead. These should be installed in the order they appear in the configuration file.

11. [ ] if a `manifests` folder exists at helm/charts/$chart_name/manifests that contains  one or more valid yaml files then that folder will be used with the command `kubectl apply -f helm/charts/$chart_name/manifests`

12. [ ] once this has been done for all dependencies the same process repeated for the `application` section of the config file. I will provide the config.yaml for context.

13. [ ] by default the program assumes you are installing but if the --cleanup argument is passed then it will perform the reverse and remove all uninstall all helm charts, remove all helm repositories, and remove all manifests using `kubectl delete -f helm/charts/$chart_name/manifests`

We're going to break this problem down into small steps and perform each one by one and for each step i will provide code as a starting point.

Then you will improve the code by applying industry best practices and adding unit tests where possible.

I also have a prototype of this application in bash that you can use as a reference if needed for additional context.

Here are the constraints to your work;

- Keep all code you provide split under 50 lines per-response. Then if the file is too long, split it into multiple responses and I will prompt when ready to receive them.
- All code must include comments compatible with `rustdoc`
- All function names must be snake_case and be named using the verb_noun format
- All variable names must be snake_case
- All code must be formatted using `rustfmt`
- All code must be linted using `clippy`
- All code must be tested using `cargo test`
- All code must be documented using `cargo doc`

Here is the order of file examples I will provide. Using the above information as context provide suggestions and improvements to the code where needed.

File 1. `config.yaml` the configuration file

File 2. `Cargo.toml` the cargo manifest file

File 3. `src/main.rs` the main application file

File 4. `src/args.rs` the clap argument module

File 5. `src/config.rs` the configuration file parser

File 6. `src/helm.rs` the helm module containing all helm functions.

File 7. `src/kubectl.rs` the kubectl module containing all kubectl functions.

File 8. `src/utils.rs` the utils module containing all utility functions.

Let me know when you are ready to begin _oh wise and salty one._

---

lets work through this problem.

I'm working on a rust application named `loopy` that performs the following;

if a `cleanup` variable is set it will;

- Uninstall all Helm releases.
- Remove all Helm repositories.

If the `cleanup` variable is not set it will;

- Install all Helm repositories.
- Install all Helm releases.

I am now going to provide code from the following files;

1. relevant code from `src/main.rs` calling the helm functions.
2. relevant code from `src/helm.rs` containing the helm functions.
3. relevant code from `src/utils.rs` containing the `run_command` function used to execute helm commands.

I want you to complete the functions inside of `src/helm.rs` providing unit tests, documentation, and comments where appropriate.

---

lets work through a new problem in the same program.

I'm working on a rust application named `loopy` that performs the following;

if a `cleanup` variable is set it will;

- Uninstall all kubernetes manifests from a directory using `kubectl delete -f manifests/`

if a `cleanup` variable is not set it will;

- Install the `namespace.yaml` manifest using `kubectl apply -f manifests/namespace.yaml`
- Install the `rbac.yaml` manifest using `kubectl apply -f manifests/rbac.yaml`
- Install all manifests from a directory using `kubectl apply -f manifests/`

I am now going to provide code from the following files;

1. relevant code from `src/main.rs` calling the helm functions.
2. relevant code from `src/kubectl.rs` containing the helm functions.
3. relevant code from `src/utils.rs` containing the `run_command` function used to execute helm commands.

I want you to complete the functions inside of `src/kubectl.rs` providing unit tests, documentation, and comments where appropriate.

Let me know when you are ready to receive the first file.

When providing responses, provide 1 function at a time. I will then prompt you for the next function.
