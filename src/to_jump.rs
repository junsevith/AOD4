use crate::graph::Graph;

pub fn jump_max_flow(graph: Graph) -> String {
    let mut printable = String::new();
    printable += "import Pkg\n";
    printable += "Pkg.add(\"JuMP\")\n";
    printable += "Pkg.add(\"GLPK\")\n";
    printable += "using JuMP
using GLPK\n";
    let mut matrix = vec![vec![0; graph.vertices.len()]; graph.vertices.len()];
    for (v, edges) in graph.vertices.iter().map(|v| &v.edges).enumerate() {
        for edge in edges {
            matrix[v][edge.destination] = edge.weight;
        }
    }
    printable += "G = [\n";
    for v in matrix {
        printable += "  ";
        for u in v {
            printable += &u.to_string();
            printable += " ";
        }
        printable += "\n";
    }
    printable += "]\n";
    printable += "n = size(G)[1]\n
max_flow = Model(GLPK.Optimizer)\n\
set_optimizer_attribute(max_flow, \"msg_lev\", GLPK.GLP_MSG_ALL)\n
@variable(max_flow, f[1:n, 1:n] >= 0)\n
# Capacity constraints\n
@constraint(max_flow, [i = 1:n, j = 1:n], f[i, j] <= G[i, j])\n";
    printable += &format!(
        "@constraint(max_flow, [i = 1:n; i != 1 && i != {}], sum(f[i, :]) == sum(f[:, i]))\n",
        graph.vertices.len()
    );
    printable += "# Flow conservation constraints\n
@objective(max_flow, Max, sum(f[1, :]))\n
optimize!(max_flow)\n
println(\"Calculated max flow:\", objective_value(max_flow))\n";
    printable
}

pub fn jump_matching(graph: Graph) -> String {
    let mut printable = String::new();
    printable += "import Pkg
    Pkg.add(\"JuMP\")
    Pkg.add(\"GLPK\")
    using JuMP
    using GLPK\n";
    let mut matrix = vec![vec![0; graph.vertices.len()]; graph.vertices.len()];
    for (v, edges) in graph.vertices.iter().map(|v| &v.edges).enumerate() {
        for edge in edges {
            if v <= edge.destination {
                matrix[v][edge.destination] = edge.weight;
            }
        }
    }
    printable += "G = [\n";
    for v in matrix {
        printable += "  ";
        for u in v {
            printable += &u.to_string();
            printable += " ";
        }
        printable += "\n";
    }
    printable += "]\n";
    printable += "n = size(G)[1]\n
matching = Model(GLPK.Optimizer)\n\
set_optimizer_attribute(matching, \"msg_lev\", GLPK.GLP_MSG_ALL)\n
@variable(matching, y[1:n, 1:n], Bin)\n
# One person can only be assigned to one object
@constraint(matching, [i = 1:n], sum(y[:, i]) <= 1)
# One object can only be assigned to one person
@constraint(matching, [j = 1:n], sum(y[j, :]) <= 1)
@constraint(matching, [i = 1:n, j = 1:n], y[i, j] <= G[i, j])
@objective(matching, Max, sum(y))
optimize!(matching)\n
println(\"Calculated maximum matching size: \", objective_value(matching))\n\
println(\"Matching:\")\n\
display(value.(y))";
    printable
}
