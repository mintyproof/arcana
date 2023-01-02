pub type Tile = u8;

const SIZE_X: usize = 16;
const SIZE_Y: usize = 16;
const SIZE_Z: usize = 16;

pub struct World {
    size: (usize, usize, usize),
    tiles: Vec<Tile>,
}

impl World {
    pub fn new() -> Self {
        let size = (SIZE_X, SIZE_Y, SIZE_Z);
        let mut tiles = Vec::new();
        tiles.resize(SIZE_X * SIZE_Y * SIZE_Z, 0);

        Self { size, tiles }
    }

    pub fn raycast() -> Tile {
        /* glsl prototype code:

        ivec3 tPos = ivec3(floor(rayPos));
        ivec3 tStep;
        tStep.x = rayDir.x < 0.0 ? -1 : 1;
        tStep.y = rayDir.y < 0.0 ? -1 : 1;
        tStep.z = rayDir.z < 0.0 ? -1 : 1;
        vec3 tDelta = abs(1.0 / rayDir);
        vec3 tMax;
        tStep.x < 0 ? tMax.x = (rayPos.x - float(tPos.x)) * tDelta.x
                    : tMax.x = (float(tPos.x) + 1.0 - rayPos.x) * tDelta.x;
        tStep.y < 0 ? tMax.y = (rayPos.y - float(tPos.y)) * tDelta.y
                    : tMax.y = (float(tPos.y) + 1.0 - rayPos.y) * tDelta.y;
        tStep.z < 0 ? tMax.z = (rayPos.z - float(tPos.z)) * tDelta.z
                    : tMax.z = (float(tPos.z) + 1.0 - rayPos.z) * tDelta.z;

        int tile = 0, stepsTaken = 0;
        vec3 mask;
        do {
            stepsTaken += 1;

            mask = step(tMax.xyz, tMax.yzx) * step(tMax.xyz, tMax.zxy);
            tMax += tDelta * mask;
            tPos += tStep * ivec3(mask);

            tile = get(tPos.x, tPos.y, tPos.z);
        } while (tile == 0 && stepsTaken < renderLimit);

        return tile;
        */
        0
    }

    pub fn in_bounds(&self, x: usize, y: usize, z: usize) -> bool {
        /* x >= 0 && y >= 0 && z >= 0 && */
        x < self.size.0 && y < self.size.1 && z < self.size.2
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, tile: Tile) {
        let index = self.offset_of(x, y, z);
        self.tiles[index] = tile;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<Tile> {
        if !self.in_bounds(x, y, z) {
            return None;
        }

        let index = self.offset_of(x, y, z);
        Some(self.tiles[index])
    }

    pub fn size_x(&self) -> usize {
        self.size.0
    }

    pub fn size_y(&self) -> usize {
        self.size.1
    }

    pub fn size_z(&self) -> usize {
        self.size.2
    }

    fn offset_of(&self, x: usize, y: usize, z: usize) -> usize {
        (self.size.0 * self.size.1 * z) + (self.size.0 * y) + x
    }
}
