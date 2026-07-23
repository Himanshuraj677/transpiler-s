use std::fs::File;
use std::io::{BufWriter, Write};

use transpiler_scratch::Lexer;

fn main() -> std::io::Result<()> {
    // Correct Program
    let programs = [
        (
            "Bell State",
            r#"OPENQASM 2.0;
include "qelib1.inc";

qreg q[2];
creg c[2];

h q[0];
cx q[0], q[1];

measure q[0] -> c[0];
measure q[1] -> c[1];
"#,
        ),
        (
            "Single Qubit",
            r#"OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

x q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "GHZ State",
            r#"OPENQASM 2.0;
include "qelib1.inc";

qreg q[3];
creg c[3];

h q[0];
cx q[0], q[1];
cx q[1], q[2];

measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
"#,
        ),
        (
            "Custom Gate Definition",
            r#"OPENQASM 2.0;
include "qelib1.inc";
gate my_gate(theta, phi) a, b {
    u3(theta, phi, -pi/2) a;
    cx a, b;
    u1(phi + theta) b;
}
qreg q[2];
creg c[2];
my_gate(pi/4, pi/2) q[0], q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
"#,
        ),
        (
            "Opaque Gate Declaration",
            r#"OPENQASM 2.0;
include "qelib1.inc";
opaque black_box(a, b) q0, q1;
qreg q[2];
creg c[2];
black_box(0.5, -1.25) q[0], q[1];
measure q[0] -> c[0];
"#,
        ),
        (
            "Classical If Statement",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
creg c[2];
h q[0];
measure q[0] -> c[0];
if(c==1) x q[1];
measure q[1] -> c[1];
"#,
        ),
        (
            "Barrier Directive",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];
h q[0];
h q[1];
h q[2];
barrier q[0], q[1], q[2];
cx q[0], q[1];
measure q[0] -> c[0];
"#,
        ),
        (
            "Nested Math Expressions",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
creg c[1];
u3(sin(pi/4) + cos(pi/3), sqrt(2) * exp(1), ln(2.71828)) q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "Negative and Scientific Notation Reals",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
creg c[1];
rz(-1.5707963) q[0];
rx(3.14e-2) q[0];
ry(-2.5E+1) q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "Multiple Registers",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg a[2];
qreg b[3];
creg ca[2];
creg cb[3];
h a[0];
cx a[0], b[0];
cx b[0], b[1];
measure a[0] -> ca[0];
measure b[1] -> cb[1];
"#,
        ),
        (
            "Toffoli Decomposition",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];
h q[2];
cx q[1], q[2];
tdg q[2];
cx q[0], q[2];
t q[2];
cx q[1], q[2];
tdg q[2];
cx q[0], q[2];
t q[1];
t q[2];
cx q[0], q[1];
h q[2];
t q[0];
tdg q[1];
cx q[0], q[1];
measure q[2] -> c[2];
"#,
        ),
        (
            "Controlled Phase Gates",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];
cu1(pi/2) q[0], q[1];
cu1(pi/4) q[0], q[2];
cu1(pi/2) q[1], q[2];
h q[2];
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
"#,
        ),
        (
            "Comments and Whitespace Edge Cases",
            r#"OPENQASM 2.0;
// This is a line comment
include "qelib1.inc"; // another comment
qreg   q[2] ;
creg c[2];


h q[0];    // apply hadamard
cx q[0],q[1];// no space before comment
measure q[0]   ->   c[0] ;
"#,
        ),
        (
            "Deep Parenthesized Expression",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
creg c[1];
u1(((pi/2) + (pi/4)) * (2 - 1)) q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "Multiple Gate Definitions",
            r#"OPENQASM 2.0;
include "qelib1.inc";
gate bell a, b {
    h a;
    cx a, b;
}
gate unbell a, b {
    cx a, b;
    h a;
}
qreg q[2];
creg c[2];
bell q[0], q[1];
unbell q[0], q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
"#,
        ),
        (
            "QFT Three Qubit",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];
h q[0];
cu1(pi/2) q[1], q[0];
cu1(pi/4) q[2], q[0];
h q[1];
cu1(pi/2) q[2], q[1];
h q[2];
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];
"#,
        ),
        (
            "String Literal With Path",
            r#"OPENQASM 2.0;
include "qelib1.inc";
include "custom_gates/my_lib.inc";
qreg q[1];
creg c[1];
x q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "U Gate Family",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
creg c[1];
u1(pi/2) q[0];
u2(0, pi) q[0];
u3(pi/2, 0, pi) q[0];
U(pi/4, pi/8, pi/16) q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "Multiple If Statements",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];
h q[0];
h q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
if(c==1) x q[2];
if(c==2) z q[2];
if(c==3) y q[2];
measure q[2] -> c[2];
"#,
        ),
        (
            "Recursive-Style Gate Composition",
            r#"OPENQASM 2.0;
include "qelib1.inc";
gate layer1 a { h a; t a; }
gate layer2 a { layer1 a; layer1 a; }
qreg q[1];
creg c[1];
layer2 q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "Empty Gate Body",
            r#"OPENQASM 2.0;
include "qelib1.inc";
gate noop a {
}
qreg q[1];
creg c[1];
noop q[0];
x q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "Trig and Inverse Trig Mix",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
creg c[1];
rx(asin(0.5)) q[0];
ry(acos(-0.5)) q[0];
rz(atan(1.0)) q[0];
measure q[0] -> c[0];
"#,
        ),
        (
            "Large Register Indices",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[16];
creg c[16];
h q[0];
cx q[0], q[15];
cx q[7], q[8];
measure q[0] -> c[0];
measure q[15] -> c[15];
"#,
        ),
        (
            "No Trailing Newline Program",
            r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
creg c[1];
x q[0];
measure q[0] -> c[0];"#,
        ),
    ];

// let invalid_programs = [
//     (
//         "Invalid Character",
//         r#"OPENQASM 2.0;

// @
// "#,
//     ),
//     (
//         "Multiple Invalid Symbols",
//         r#"$
// &
// #
// ~
// `
// "#,
//     ),
//     (
//         "Invalid Identifier",
//         r#"OPENQASM 2.0;

// meas$ure q[0];
// "#,
//     ),
//     (
//         "Unterminated String",
//         r#"OPENQASM 2.0;
// include "qelib1.inc;
// "#,
//     ),
//     (
//         "Empty String",
//         r#"OPENQASM 2.0;
// include "";
// "#,
//     ),
//     (
//         "Malformed Real",
//         r#"OPENQASM 2.0;

// 3..14
// "#,
//     ),
//     (
//         "Missing Exponent",
//         r#"OPENQASM 2.0;

// 1e
// "#,
//     ),
//     (
//         "Missing Exponent Sign Digits",
//         r#"OPENQASM 2.0;

// 1e+
// "#,
//     ),
//     (
//         "Missing Negative Exponent Digits",
//         r#"OPENQASM 2.0;

// 1e-
// "#,
//     ),
//     (
//         "Hex Literal",
//         r#"OPENQASM 2.0;

// 0xFF
// "#,
//     ),
//     (
//         "Binary Literal",
//         r#"OPENQASM 2.0;

// 0b1010
// "#,
//     ),
//     (
//         "Unicode Identifier",
//         "OPENQASM 2.0;\n\nπ q[0];\n",
//     ),
//     (
//         "Broken Arrow",
//         r#"OPENQASM 2.0;

// measure q[0] - > c[0];
// "#,
//     ),
//     (
//         "Triple Equals",
//         r#"OPENQASM 2.0;

// if(c===1) x q[0];
// "#,
//     ),
//     (
//         "Single Equals",
//         r#"OPENQASM 2.0;

// if(c=1) x q[0];
// "#,
//     ),
//     (
//         "Invalid Operator",
//         r#"OPENQASM 2.0;

// ->>
// "#,
//     ),
//     (
//         "Missing Right Bracket",
//         r#"OPENQASM 2.0;

// qreg q[2;
// "#,
//     ),
//     (
//         "Missing Right Parenthesis",
//         r#"OPENQASM 2.0;

// u3(pi/2, pi
// "#,
//     ),
//     (
//         "Very Long Identifier",
//         r#"OPENQASM 2.0;

// averyveryveryveryveryveryveryveryveryveryveryveryveryverylongidentifier
// "#,
//     ),
//     (
//         "Comment at EOF",
//         r#"OPENQASM 2.0;
// // comment without newline"#,
//     ),
//     (
//         "Mixed Garbage",
//         "OPENQASM 2.0;

// @$%

// qreg q[2;

// measure q[0] => c[0];

// include \"unterminated

// 1e+
// 3..14

// π

// meas$ure q[0];

// ==>
// ",
//     ),
// ];

    let file = File::create("lexer_output.txt")?;
    let mut writer = BufWriter::new(file);

    for (i, (name, qasm)) in programs.iter().enumerate() {
        writeln!(writer, "Program {}: {}", i + 1, name)?;
        writeln!(writer, "{}", "=".repeat(80))?;
        writeln!(writer)?;
        writeln!(writer, "{}", qasm)?;

        writeln!(
            writer,
            "{:<16} {:<20} {:<8} {}",
            "Kind", "Lexeme", "Span", "Location"
        )?;
        writeln!(writer, "{}", "-".repeat(60))?;

        let lexer = Lexer::new(qasm);

        for token in lexer {
            writeln!(writer, "{}", token)?;
        }

        writeln!(writer)?;
        writeln!(writer, "{}", "=".repeat(80))?;
        writeln!(writer)?;
    }

    println!("Lexer output written to lexer_output.txt");

    Ok(())
}