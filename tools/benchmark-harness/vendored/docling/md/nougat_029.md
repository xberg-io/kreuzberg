<!-- image -->

## Bispindle in strongly connected digraphs with large chromatic number

Nathann Cohen, Frédéric Havet, William Lochet, Raul Lopes

## To cite this version

Nathann Cohen, Frédéric Havet, William Lochet, Raul Lopes. Bispindle in strongly connected digraphs with large chromatic number. Electronic Notes in Discrete Mathematics, Elsevier, 2017, 62, pp.69 - 74. ￿10.1016/j.endm.2017.10.013￿. ￿hal-01634307￿

## HAL Id: hal-01634307 <https://hal.inria.fr/hal-01634307>

Submitted on 13 Nov 2017

HAL is a multi-disciplinary open access archive for the deposit and dissemination of scientific research documents, whether they are published or not. The documents may come from teaching and research institutions in France or abroad, or from public or private research centers.

L'archive ouverte pluridisciplinaire HAL , est destinée au dépôt et à la diffusion de documents scientifiques de niveau recherche, publiés ou non, émanant des établissements d'enseignement et de recherche français ou étrangers, des laboratoires publics ou privés.

## Bispindle in strongly connected digraphs with large chromatic number

Nathann Cohen 1

CNRS, LRI, Univ. Paris Sud, Orsay, France

Fr´ ed´ eric Havet 2

Univ. Cˆ ote d'Azur, CNRS, I3S, INRIA, France

William Lochet 3

Univ. Cˆ ote d'Azur, CNRS, I3S, INRIA, and LIP, ENS Lyon, France

Raul Lopes 4

Departamento de Computa¸ cao, Universidade Federal do Cear´ a, Fortaleza, Brazil

## Abstract

A ( k 1 + k 2 ) -bispindle is the union of k 1 ( x, y )-dipaths and k 2 ( y, x )-dipaths, all these dipaths being pairwise internally disjoint. Recently, Cohen et al. showed that for every (2 + 0)- bispindle B , there exists an integer k such that every strongly connected digraph with chromatic number greater than k contains a subdivision of B . We investigate generalisations of this result by first showing constructions of strongly connected digraphs with large chromatic number without any (3 + 0)bispindle or (2+2)-bispindle. Then we show that for any k , there exists γ k such that every strongly connected digraph with chromatic number greater than γ k contains a (2 + 1)-bispindle with the ( y, x )-dipath and one of the ( x, y )-dipaths of length at least k .

Keywords: Digraph, chromatic number, subdivision.

## 1 Introduction

Throughout this paper, the chromatic number of a digraph D , denoted by χ ( D ), is the chromatic number of its underlying graph. In a digraph D , a directed path , or dipath , is an oriented path where all the arcs are oriented form the initial vertex towards the terminal vertex. A k -spindle is the union of k internally disjoint ( x, y )-dipaths for some vertices x and y . Vertex x is said to be the tail of the spindle and y its head . A ( k 1 + k 2 ) -bispindle is the internally disjoint union of a k 1 -spindle with tail x and head y and a k 2 -spindle with tail y and head x . In other words, it is the union of k 1 ( x, y )-dipaths and k 2 ( y, x )-dipaths, all of these dipaths being pairwise internally disjoint.

A classical result due to Gallai, Hasse, Roy and Vitaver is the following.

```text
Theorem 1.1 (Gallai [8], Hasse [9], Roy [11], Vitaver [12]) If χ ( D ) ≥ k , then D contains a dipath of length k -1 .
```

This raises the question of which digraphs are subdigraphs of all digraphs with large chromatic number.

A classical theorem by Erd˝ os [6] implies that if H is a digraph containing a cycle, there exist digraphs with arbitrarily high chromatic number with no subdigraph isomorphic to H . Thus the only possible candidates to generalise Theorem 1.1 are the oriented trees that are orientations of trees. Burr[3] proved that every ( k -1) 2 -chromatic digraph contains every oriented tree of order k and conjectured an upper bound of 2 k -2. The best known upper bound, due to Addario-Berry et al. [1], is in ( k/ 2) 2 .

However the following celebrated theorem of Bondy shows that the story does not stop there.

Theorem 1.2 (Bondy [2]) Every strongly connected digraph with chromatic number at least k contains a directed cycle of length at least k .

1 <nathann.cohen@gmail.com>

2 <frederic.havet@cnrs.fr>

3 <william.lochet@gmail.com>

4 <raul.wayne@gmail.com>

The strong connectivity assumption is indeed necessary, as transitive tournaments contain no directed cycle but can have arbitrarily high chromatic number.

Observe that a directed cycle of length at least k can be seen as a subdivision of /vector C k , the directed cycle of length k . Recall that a subdivision of a digraph F is a digraph that can be obtained from F by replacing each arc uv by a dipath from u to v .

Conjecture 1.3 (Cohen et al. [5]) For every cycle C , there exists a constant f ( C ) such that every strongly connected digraph with chromatic number at least f ( C ) contains a subdivision of C .

The strong connectivity assumption is also necessary in Conjecture 1.3 as shown by Cohen et al. in [5]. In the same paper, Conjecture 1.3 was confirmed for cycles with two blocks (i.e. maximal subdipaths of the cycle) and the antidirected cycle of length 4. More precisely, denoting by C ( k, /lscript ) the cycle on two blocks, one of length k and the other of length /lscript , Cohen et al. [5] proved the following.

Theorem 1.4 Every strongly connected digraph with chromatic number at least O (( k + /lscript ) 4 ) contains a subdivision of C ( k, /lscript ) .

The bound has recently been improved to O (( k + /lscript ) 2 ) by Kim et al. [10]. A subdivision of C ( k, /lscript ) can be seen as a 2-spindle made of two internally disjoint dipaths, one of length at least k and one of length at least /lscript . In this paper, we generalize this and study the existence of subdivision of spindles and bispindles in strongly connected digraphs with large chromatic number. Our first result is to give constructions for the following theorem:

Theorem 1.5 For every integer k , there exists a strongly connected digraph D with χ ( D ) &gt; k that contains no 3 -spindle and no (2 + 2) -bispindle.

Therefore, the most we can expect in all strongly connected digraphs with large chromatic number are (2 + 1)-bispindle. Let B ( k 1 , k 2 ; k 3 ) denote the (2+1)-bispindle formed by three internally disjoint paths between two vertices x, y , two ( x, y )-dipaths, one of size k 1 , the other of size k 2 , and one ( y, x )-dipath of size k 3 . We conjecture the following.

Conjecture 1.6 There is a function g : N 3 → N such that every strongly connected digraph with chromatic number at least g ( k 1 , k 2 , k 3 ) contains a subdivision of B ( k 1 , k 2 ; k 3 ) .

As an evidence, we prove the following theorem:

Theorem 1.7 For every positive integer k , there is a constant γ k such that every strongly connected digraph witch chromatic number greater than γ k contains a subdivsion of B ( k, 1; k ) .

The value of γ k is the above theorem is huge, and certainly not best possible. We get a better bound for subdivision of B ( k, 1; k ).

Theorem 1.8 Let k ≥ 3 be an integer and let D be a strong digraph. If χ ( D ) &gt; (2 k -2)(2 k -3) , then D contains a subdivision of B ( k, 1; 1) .

## 2 Proof of Theorem 1.7

Weprove Theorem 1.7 by the contrapositive. We consider a digraph D without any subdivision of B ( k, 1; k ). We shall prove that χ ( D ) ≤ γ k .

The general idea is to use the following easy lemma.

Lemma 2.1 Let D be a digraph, D 1 . . . D l be disjoint subdigraphs of D and D ′ the digraph obtained by contracting each D i into one vertex d i . Then χ ( D ) ≤ χ ( D ′ ) · max { χ ( D i ) | 1 ≤ i ≤ l } .

The key is to find appropriate subdigraphs D i . To do so, we consider some particular collections of directed cycles : a collection C of directed cycles is k -suitable if all cycles of C have length at least 8 k , and any two distinct cycles C i , C j ∈ C intersect on a subpath of order at most k . A component of C is a connected component of the underlying graph of the digraph ⋃ C which is the union of cycles of C .

Consider C be a maximal k -suitable collection of cycles in D . Let D ′ be the digraph obtained by contracting every strong component S of ⋃ C (which is ⋃ S for some component S of C ) into one vertex. For each connected component S i we call s i the new vertex created. To apply Lemma 2.1, we shall prove in the next two lemmas that for every component S of C , the digraph D [ S ] induced by D on the vertices of ⋃ S has bounded chromatic number and that χ ( D ′ ) ≤ 8 k .

Lemma 2.2 Let C be a k -suitable collection of directed cycles in a B ( k, 1; k ) -free digraph D . There exists a constant β k such that, for every component S of C , we have χ ( D [ S ]) ≤ β k .

Sketch of proof : We first consider ⋃ S which is a subdigraph of D [ S ]. We prove by induction on the number of cycles in S that this digraph admits a proper colouring φ with α k = 2 · (6 k 2 ) 3 k +14 k colours satisfying the following additional property, called rainbow property : the vertices of each subpath of length at most 7 k of each cycle of S get different colours.

We then define a sort of Breadth-First-Search for S . Let C 0 be a cycle of S and set L 0 = { C 0 } . We build the levels L i inductively until all cycles of S are put in a level : L i +1 consists of every cycle C l not in ⋃ j ≤ i L j such that there exists a cycle in L i intersecting C l . For every C l ∈ L i +1 , we choose one of the cycles L i intersecting it to be its father . For a vertex x of ⋃ S , we say that x belongs to level L i if i is the smallest integer such that there exists a cycle in L i containing x .

We partition the arc set of D [ S ] in ( A 0 , A 1 , A 2 ), where

- A 0 is the set of arcs of D [ S ] which ends belong to the same level, and
- A 1 is the set of arcs of D [ S ] which ends belong to different levels i and j with | i -j | &lt; k .
- A 2 is the set of arcs of D [ S ] which ends belong to different levels i and j with | i -j | ≥ k .

For i ∈ { 0 , 1 , 2 } , let D i be the spanning subdigraph of D [ S ] with arc set A i . It is well-known that χ ( D [ S ]) ≤ χ ( D 0 ) × χ ( D 1 ) × χ ( D 2 ).

Clearly, χ ( D 1 ) ≤ k , and we show that χ ( D 2 ) ≤ 4 k 2 +2. To bound χ ( D 0 ) we partition the vertex set according to the above-mention colouring φ of ⋃ S . Using the rainbow property, we prove that the subdigraph of D 0 induced by the vertices of colour c has chromatic number at most 2 · (4 k ) 4 k + 1 for all colour c . Hence χ ( D 0 ) ≤ (2 · (4 k ) 4 k + 1) α k . This gives the result for β k = k (4 k 2 +2)(2 · (4 k ) 4 k +1) α k . /square

## Lemma 2.3 χ ( D ′ ) ≤ 8 k

Proof. First note that since D is strongly connected so is D ′ .

Suppose for a contradiction that χ ( D ′ ) &gt; 8 k . By Theorem 1.2, there exists a directed cycle C ′ = ( x 1 , x 2 , . . . , x l , x 1 ) of length at least 8 k . For each vertex x j that corresponds to a S i in D , the arc x j -1 x j corresponds in D to an arc whose head is a vertex p i of S i and the arc x j x j +1 corresponds to an arc whose tail is a vertex l i of S i . Let P j be the dipath from p i to l i in ⋃ C . Note that this path intersects the elements of S i only along a subdipath. Let C be the cycle obtained from C ′ where we replace all contracted vertices x j by the path P j . First note that C has length at least 8 k . Moreover, a cycle of C can intersect C only along one P j , because they all correspond to different strong components of ⋃ C . Thus C intersects each cycle of C on a subdipath. Moreover this subdipath has length smaller than k for otherwise D would contain a subdivision of B ( k, 1; k ). So C is a directed cycle of length at least

8 k which intersects every cycle of C along a subdipath of length less than k . This contradicts the maximality of C . /square

Using Lemma 2.1 with Claim 2.3 and Lemma 2.2, we get that χ ( D ) ≤ 8 k · β k . This proves Theorem 1.7 for γ k = 8 k · β k .

## References

- [1] L. Addario-Berry, F. Havet, C. L. Sales, B. A. Reed, and S. Thomass´ e. Oriented trees in digraphs. Discrete Mathematics , 313 (8): 967-974, 2013.
- [2] J. A. Bondy, Disconnected orientations and a conjecture of Las Vergnas, J. London Math. Soc. (2) , 14 (2): 277-282, (1976).
- [3] S. A. Burr. Subtrees of directed graphs and hypergraphs. In Proceedings of the 11th Southeastern Conference on Combinatorics, Graph theory and Computing , pages 227-239, Boca Raton - FL, 1980. Florida Atlantic University.
- [4] S. A. Burr, Antidirected subtrees of directed graphs. Canad. Math. Bull. 25 : 1982 119-120, 1982
- [5] N. Cohen, F. Havet, W. Lochet, and N. Nisse. Subdivisions of oriented cycles in digraphs with large chromatic number. arXiv:1605.07762
- [6] P. Erd˝ os. Graph theory and probability. Canad. J. Math. , 11:34-38, 1959.
- [7] P. Erd˝ os and A. Hajnal. On chromatic number of graphs and set-systems. Acta Mathematica Academiae Scientiarum Hungarica , 17(1-2):61-99, 1966.
- [8] T. Gallai. On directed paths and circuits. In Theory of Graphs (Proc. Colloq. Titany, 1966) , pages 115-118. Academic Press, New York, 1968.
- [9] M. Hasse. Zur algebraischen bergr¨ und der graphentheorie I. Math. Nachr. , 28: 275-290, 1964.
- [10] R. Kim, SJ. Kim, J. Ma; B. Park Cycles with two blocks in k -chromatic digraphs arXiv:1610.05839
- [11] B. Roy. Nombre chromatique et plus longs chemins d'un graphe. Rev. Francaise Informat. Recherche Op´ erationnelle , 1 (5): 129-132, 1967.
- [12] L. M. Vitaver. Determination of minimal coloring of vertices of a graph by means of boolean powers of the incidence matrix. Doklady Akademii Nauk SSSR , 147: 758-759, 1962.
