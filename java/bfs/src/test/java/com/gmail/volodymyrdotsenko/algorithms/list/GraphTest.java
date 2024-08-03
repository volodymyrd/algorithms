package com.gmail.volodymyrdotsenko.algorithms.list;


import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

class GraphTest {
    private Graph graph;

    @BeforeEach
    void setUp() {
        graph = new Graph(List.of(
                new char[]{'r', 's'},
                new char[]{'r', 't'},
                new char[]{'r', 'w'},
                new char[]{'s', 'u'},
                new char[]{'s', 'v'},
                new char[]{'t', 'u'},
                new char[]{'u', 'y'},
                new char[]{'v', 'w'},
                new char[]{'v', 'y'},
                new char[]{'w', 'x'},
                new char[]{'w', 'z'},
                new char[]{'x', 'y'},
                new char[]{'x', 'z'}));
    }

    @Test
    void testBfs() {
        assertThat(graph.bfs('s'))
                .containsExactlyInAnyOrder(
                        new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null)),
                        new Graph.Vertex('s', true, 0, null),
                        new Graph.Vertex('t', true, 2, new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null))),
                        new Graph.Vertex('u', true, 1, new Graph.Vertex('s', true, 0, null)),
                        new Graph.Vertex('v', true, 1, new Graph.Vertex('s', true, 0, null)),
                        new Graph.Vertex('w', true, 2, new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null))),
                        new Graph.Vertex('x', true, 3, new Graph.Vertex('w', true, 2, new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null)))),
                        new Graph.Vertex('y', true, 2, new Graph.Vertex('u', true, 1, new Graph.Vertex('s', true, 0, null))),
                        new Graph.Vertex('z', true, 3, new Graph.Vertex('w', true, 2, new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null)))));
    }

    @Test
    void testShortestPath() {
        assertThat(graph.shortestPath('s', 'x')).isEqualTo(List.of(
                new Graph.Vertex('s', true, 0, null),
                new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null)),
                new Graph.Vertex('w', true, 2, new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null))),
                new Graph.Vertex('x', true, 3, new Graph.Vertex('w', true, 2, new Graph.Vertex('r', true, 1, new Graph.Vertex('s', true, 0, null))))));
    }
}
