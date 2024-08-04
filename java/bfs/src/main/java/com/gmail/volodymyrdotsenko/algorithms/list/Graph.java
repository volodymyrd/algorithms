package com.gmail.volodymyrdotsenko.algorithms.list;

import java.util.*;

public final class Graph {

    public static final class Vertex {
        private final char label;
        private boolean visited = false;
        private int dist = Integer.MAX_VALUE;
        private Vertex pi;

        public Vertex(char label) {
            this.label = label;
        }

        public Vertex(char label, boolean visited, int dist, Vertex pi) {
            this.label = label;
            this.visited = visited;
            this.dist = dist;
            this.pi = pi;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Vertex vertex = (Vertex) o;
            return label == vertex.label && visited == vertex.visited
                    && dist == vertex.dist && Objects.equals(pi, vertex.pi);
        }

        @Override
        public int hashCode() {
            return Objects.hash(label, visited, dist, pi);
        }

        @Override
        public String toString() {
            return "Vertex{" +
                    "label=" + label +
                    ", visited=" + visited +
                    ", dist=" + dist +
                    ", pi=" + (pi != null ? pi.label : "-") +
                    '}';
        }
    }

    private final Map<Character, Vertex> vertices = new HashMap<>();
    private final Map<Character, List<Vertex>> adj = new HashMap<>();

    public Graph(List<char[]> edges) {
        for (var edge : edges) {
            var src = vertices.getOrDefault(edge[0], new Vertex(edge[0]));
            vertices.putIfAbsent(edge[0], src);
            var dest = vertices.getOrDefault(edge[1], new Vertex(edge[1]));
            vertices.putIfAbsent(edge[1], dest);
            List<Vertex> l1 = adj.getOrDefault(edge[0], new ArrayList<>());
            l1.add(dest);
            adj.put(edge[0], l1);
            List<Vertex> l2 = adj.getOrDefault(edge[1], new ArrayList<>());
            l2.add(src);
            adj.put(edge[1], l2);
        }
    }

    public List<Vertex> shortestPath(char source, char target) {
        if (target == source) {
            return List.of(new Vertex(target, true, 0, null));
        }
        bfs(source, target);
        List<Vertex> path = new ArrayList<>();
        printPath(source, vertices.get(target), path);
        Collections.reverse(path);
        return path;
    }

    private void printPath(char source, Vertex target, List<Vertex> path) {
        if (target == null) {
            return;
        }
        path.add(target);
        if (target.pi == null || target.label == source) {
            return;
        }
        printPath(source, target.pi, path);
    }

    public List<Vertex> bfs(char source) {
        return bfs(source, null);
    }

    private List<Vertex> bfs(char source, Character target) {
        Vertex src = vertices.get(source);
        src.visited = true;
        src.dist = 0;
        Queue<Vertex> queue = new LinkedList<>();
        queue.add(src);
        while (!queue.isEmpty()) {
            var u = queue.poll();
            for (var v : adj.get(u.label)) {
                if (!v.visited) {
                    v.visited = true;
                    v.dist = u.dist + 1;
                    v.pi = u;
                    if (target != null && v.label == target) {
                        break;
                    }
                    queue.add(v);
                }
            }
        }
        return vertices.values().stream().toList();
    }
}

