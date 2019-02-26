#include   <stdio.h>
#include   <math.h>
#include   <stdlib.h>
#include   <time.h>
#include <emscripten/emscripten.h>


struct Body {
  double x;
  double y;
  double vx;
  double vy;
  double mass;
};

struct Graph {
  struct Body *bodies;
  int n;
};

static struct Graph G;
static int N = 10;

#ifdef __cplusplus
extern "C" {
#endif

void print (struct Graph *g) {
  for (int i = 0; i < g->n; i++) {
    struct Body bi = g->bodies[i];
    printf("%d %f %f %f %f\n", i, bi.x, bi.y, bi.vx, bi.vy);
  }
}

void advance (struct Graph *g, double dt) {
  int n = g->n;
  for (int i = 0; i < n; i++) {
    struct Body *bi = &g->bodies[i];
    for (int j = i + 1; j < n; j++) {
      struct Body *bj = &g->bodies[j];
      double dx = bi->x - bj->x;
      double dy = bi->y - bj->y;

      double d2  = dx * dx + dy * dy;
      double mag = dt / (d2 * sqrt(d2));

      double massj_mag = bj->mass * mag;
      bi->vx = bi->vx - dx * massj_mag;
      bi->vy = bi->vy - dy * massj_mag;

      double massi_mag = bi->mass * mag;
      bj->vx = bj->vx + dx * massi_mag;
      bj->vy = bj->vy + dy * massi_mag;
      // printf("%d:%d %f %f %f %f\n", i, j, dx, dy, bi->vx, bi->vy);
    }
    bi->x = bi->x + dt * bi->vx;
    bi->y = bi->y + dt * bi->vy;
  }
}


double energy (struct Graph *g) {
  double e = 0.0;
  for (int i = 0; i < g->n; i++) {
    struct Body bi = g->bodies[i];
    //printf("%d %f %f\n", i, bi.vx, bi.vy);
    e += (bi.vx * bi.vx + bi.vy * bi.vy) * bi.mass / 2.0;

    for (int j = i + 1; j < g->n; j++) {
      struct Body bj = g->bodies[j];
      double dx = bi.x - bj.x;
      double dy = bi.y - bj.y;
      double dist = hypot(dx, dy);
      e -= bi.mass * bj.mass / dist;
    }
  }
  return e;
}


void EMSCRIPTEN_KEEPALIVE init(int argc, char ** argv) {
  struct Graph *self = &G;
  self->bodies  = malloc(sizeof(*self->bodies) * N);
  self->n = N;
  time_t t;
  srand((unsigned) time(&t));

  for (int i = 0; i < N; i ++) {
    self->bodies[i].x = (double)rand() / RAND_MAX;
    self->bodies[i].y = (double)rand() / RAND_MAX;
    self->bodies[i].vx = 0.0;
    self->bodies[i].vy = 0.0;
    self->bodies[i].mass = 5.0;
  }
}


double EMSCRIPTEN_KEEPALIVE step() {
  //printf("step called\n");
  advance(&G, 0.01);
  //print(&G);
  return energy(&G);
}


void EMSCRIPTEN_KEEPALIVE bench(int steps) {
  printf("bench %d energy %f\n", steps, energy(&G));
  for (int i = 0; i < steps; i++) {
    advance(&G, 0.01);
    //printf("%d\n", i);
  }
  printf("energy %f\n", energy(&G));
}


int main(int argc, char ** argv) {
  //init(argc, argv);
  return 0;
}


#ifdef __cplusplus
}
#endif
