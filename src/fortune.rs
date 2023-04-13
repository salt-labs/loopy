//! Fortune Cookie
//!
//! This module provides a function to generate a random fortune message.
//!

use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Having fun with ChatGPT
/// I asked it to generate a 100 fortune cookie messages
/// that are Kubernetes toolchain related.
const FORTUNES: &[&str] = &[
    "A Carvel deployment is like an ice cream cone with all the fixings, satisfying your Kubernetes cravings in the best way possible.",
    "A Carvel deployment is like an ice cream float, combining all the best parts of your Kubernetes resources into one delicious concoction.",
    "A Carvel deployment is like an ice cream sandwich, bringing together all the best parts of your Kubernetes resources.",
    "A Carvel deployment is like an ice cream social, bringing people together around the perfect Kubernetes resources.",
    "A Carvel deployment is like an ice cream sundae with all the toppings.",
    "A Carvel deployment will make your Kubernetes resources look and taste better than ever.",
    "A Helm chart is like a treasure map, guiding you to a successful deployment.",
    "A new Kubernetes cluster will bring you joy.",
    "A rolling update gathers no downtime.",
    "A well-placed breakpoint can reveal the secrets of your code.",
    "A well-written Helm chart is worth a thousand words.",
    "Automation is the compass that leads to efficiency.",
    "Automation is the driving force behind innovation.",
    "Automation is the first step towards optimization.",
    "Carvel tools are like the perfect ice cream recipe, with just the right mix of ingredients to make your Kubernetes deployment a success.",
    "Carvel tools are like the secret recipe for the perfect Kubernetes deployment.",
    "Carvel tools are like the ultimate ice cream parlor, with everything you need to create the perfect Kubernetes deployment.",
    "Carvel tools are like the ultimate ice cream scoop, serving up perfect Kubernetes resources every time.",
    "Carvel tools are like the ultimate ice cream truck, delivering the perfect Kubernetes deployment every time.",
    "Carvel tools are the secret ingredients that will take your Kubernetes deployments to the next level.",
    "Debugging is like solving a puzzle, and the solution is just around the corner.",
    "Debugging: The art of discovering the unknown.",
    "Deliver highly available and fault-tolerant Kubernetes clusters with Tanzu Kubernetes Grid.",
    "Deploying to multiple clouds has never been easier than with Tanzu Kubernetes Grid.",
    "Efficient automation is the foundation of a successful software system.",
    "Effortlessly manage Kubernetes clusters across multiple environments with Tanzu Kubernetes Grid.",
    "Eliminate Kubernetes complexity with Tanzu Kubernetes Grid's intuitive user interface.",
    "Embrace debugging as a learning opportunity.",
    "Empower your teams with Tanzu Kubernetes Grid's self-service capabilities.",
    "Ensure compliance and security with Tanzu Kubernetes Grid's built-in policy and security controls.",
    "Ensure consistency and compatibility across all your Kubernetes clusters with Tanzu Kubernetes Grid.",
    "Ensure high availability and uptime with Tanzu Kubernetes Grid's self-healing capabilities.",
    "Ensure high performance and low latency with Tanzu Kubernetes Grid's intelligent scheduling.",
    "Experience the flexibility and agility of Tanzu Kubernetes Grid.",
    "Experience the power of Kubernetes with the simplicity of Tanzu Kubernetes Grid.",
    "Get the most out of Kubernetes with Tanzu Kubernetes Grid's advanced capabilities.",
    "Get up and running with Kubernetes in minutes with Tanzu Kubernetes Grid.",
    "Helm charts are like a box of chocolates, you never know what you're gonna get.",
    "Helm charts will pave the way to a smooth deployment journey.",
    "Helm is the guiding star of your Kubernetes deployments.",
    "Helm templating is great, but ytt is like the ultimate ice cream maker, churning out perfect Kubernetes resources every time.",
    "Helm will guide you through the stormy seas of container management.",
    "Improve application performance and scalability with Tanzu Kubernetes Grid's autoscaling capabilities.",
    "Improve your application deployment time with Tanzu Kubernetes Grid's automation capabilities.",
    "In Kubernetes, there is strength in numbers (of replicas).",
    "In the depths of Helm, you will find the treasures of chart management.",
    "In the realm of YAML, consistency is key.",
    "In the world of Kubernetes, pods come and go, but services are forever.",
    "Kapp and kapp-controller are like the ultimate ice cream sundae bar, giving you unlimited options for customizing your Kubernetes deployments.",
    "Keep your YAML clean, and your deployments will be serene.",
    "Kubernetes deployment management can be messy, but kapp and kapp-controller are like a perfect swirl of your favorite flavors, delivering a clean and delicious deployment every time.",
    "Kubernetes deployment management is like trying to scoop ice cream with a fork, but kapp and kapp-controller ensure your resources are always just right.",
    "Kubernetes deployments are like ice cream cones, and Carvel tools are the sprinkles that make them extra special.",
    "Kubernetes deployments are like ice cream sundaes, and Carvel tools are the toppings that make them extra special.",
    "Kubernetes GitOps can be messy, but kapp and kapp-controller are like a perfect swirl of your favorite flavors, delivering a clean and delicious deployment every time.",
    "Kubernetes GitOps is like a sundae with all the toppings, but kapp and kapp-controller ensure your resources are always just right.",
    "Kubernetes image management can be messy, but kbld is like a perfect swirl of your favorite flavors, delivering a clean and delicious deployment every time.",
    "Kubernetes image management is like trying to scoop ice cream with a fork, but kbld is like a perfect scoop every time, ensuring your images are always just right.",
    "Kubernetes is the glue that binds your microservices together.",
    "Kubernetes is the platform upon which your dreams will be built.",
    "Kubernetes packaging is like trying to serve ice cream on a hot day, but imgpkg ensures your resources are always fresh and ready to go.",
    "Kubernetes templating can be messy, but ytt is like a perfect swirl of your favorite flavors, delivering a clean and delicious deployment every time.",
    "Kubernetes templating is like vanilla ice cream, but ytt adds the perfect mix-ins to make it a unique and satisfying treat.",
    "Kubernetes will help you harness the power of container orchestration.",
    "Kubernetes will orchestrate your path to success.",
    "Kustomize and Helm are great, but Carvel tools are like the ultimate ice cream sundae bar, giving you unlimited options for your Kubernetes resources.",
    "Kustomize and Helm are great, but Carvel tools will take your Kubernetes resources to a whole new level of flavor and sophistication.",
    "Kustomize and Helm are great, but with Carvel tools, your Kubernetes resources will be the cherry on top of the deployment sundae.",
    "Kustomize and Helm are great, but with Carvel tools, your Kubernetes resources will be the ice cream on top of the deployment cake.",
    "Kustomize can transform your Kubernetes resources like magic.",
    "Kustomize empowers you to create dynamic Kubernetes configurations.",
    "Kustomize is the key to unlock the power of Kubernetes configurations.",
    "Kustomize will help you navigate the seas of Kubernetes customization.",
    "Kustomize will help you tailor your Kubernetes resources.",
    "Kustomize will tailor your Kubernetes resources to fit your needs.",
    "Kustomize your life and see the beauty in Kubernetes configurations.",
    "Kustomize: where YAML and Kubernetes configurations come together in harmony.",
    "Let Kubernetes manage your containers, and it will manage your success.",
    "Like a fine ice cream cake, a Carvel deployment is a treat for the senses.",
    "Managing Kubernetes resources at scale has never been simpler with Tanzu Kubernetes Grid.",
    "Maximize developer productivity and minimize downtime with Tanzu Kubernetes Grid.",
    "Maximize your Kubernetes investment with Tanzu Kubernetes Grid.",
    "Patience is a virtue, especially when waiting for a pod to become ready.",
    "Persistence in learning Helm will pay off in the long run.",
    "Reduce costs and increase efficiency with Tanzu Kubernetes Grid's infrastructure optimization.",
    "Reduce risk and ensure business continuity with Tanzu Kubernetes Grid's disaster recovery capabilities.",
    "Scale your Kubernetes clusters with ease with Tanzu Kubernetes Grid.",
    "Simplify your Kubernetes infrastructure with Tanzu Kubernetes Grid.",
    "Sometimes the best automation is the one you haven't written yet.",
    "Streamline your Kubernetes management with Tanzu Kubernetes Grid's unified control plane.",
    "Streamline your processes with automation.",
    "Tanzu Kubernetes Grid allows for easy and secure sharing of Kubernetes resources across teams.",
    "The art of debugging is the art of patience and persistence.",
    "The Carvel toolchain is a master chef that can turn your Kubernetes recipes into works of art.",
    "The container you seek is closer than it appears.",
    "The key to debugging is understanding the problem.",
    "The key to Kubernetes success lies in understanding its architecture.",
    "The kubeconfig is your passport to the Kubernetes world.",
    "The Kubernetes journey is long, but the destination is worth it.",
    "The power of Helm is in its simplicity and extensibility.",
    "The right YAML configuration can make all the difference.",
    "The solution to your Kubernetes issue is just a `kubectl` command away.",
    "The true power of Helm lies in the hands of the chart creator.",
    "There's always a bigger cluster.",
    "Upgrading your Kubernetes clusters is a breeze with Tanzu Kubernetes Grid.",
    "When debugging, remember: `kubectl logs` is your friend.",
    "With automation comes great responsibility.",
    "With Carvel tools, you can create Kubernetes resources that are as unique and delicious as a custom ice cream cake.",
    "With Carvel tools, you can design your Kubernetes resources with artisanal precision.",
    "With Carvel tools, your Kubernetes resources will be as refreshing and satisfying as a cold scoop of ice cream on a hot day.",
    "With Carvel tools, your Kubernetes resources will be as satisfying and delicious as your favorite ice cream flavor.",
    "With Carvel tools, your Kubernetes resources will be as smooth as soft-serve.",
    "With every YAML file, a new Kubernetes adventure begins.",
    "With imgpkg, offline packaging is like a perfectly frozen ice cream cake, always ready to go when you need it.",
    "With imgpkg, your Kubernetes resources are like a well-stocked ice cream truck, ready to serve up the perfect deployment every time.",
    "With kapp and kapp-controller, Kubernetes deployment management is like a perfect ice cream sandwich, bringing together all the best parts of your resources.",
    "With kapp and kapp-controller, your Kubernetes resources are like a well-stocked ice cream parlor, ready to serve up the perfect deployment every time.",
    "With kbld, Kubernetes image management is like a perfect sundae, with all the right mix-ins to make your resources shine.",
    "With ytt, Kubernetes templating is like a DIY ice cream bar, giving you unlimited options for customizing your resources.",
    "With ytt, your Kubernetes resources will be as clean and refreshing as a fresh scoop of ice cream.",
    "YAML is the blueprint of your Kubernetes kingdom.",
    "YAML is the language that brings your Kubernetes world to life.",
    "YAML, it's the fabric that weaves your Kubernetes dreams into reality.",
    "YAML: A simple format that brings complex Kubernetes resources to life.",
    "YAML: It's indentation-sensitive, just like your heart.",
];
pub struct Fortune {
    fortunes: Vec<String>,
}

impl Fortune {
    pub fn new() -> Self {
        Fortune {
            fortunes: FORTUNES.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn random_fortune(&self) -> Result<&str> {
        let mut rng = thread_rng();
        match self.fortunes.choose(&mut rng) {
            Some(fortune) => Ok(fortune),
            None => Err(anyhow!("No fortunes available")),
        }
    }
}

pub fn show_fortune() -> Option<String> {
    let fortune = Fortune::new();
    fortune.random_fortune().ok().map(|s| s.to_owned())
}
